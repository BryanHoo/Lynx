use std::sync::Arc;

use anyhow::{Result, anyhow};
use futures::AsyncReadExt as _;
use http_client::http::request;
use http_client::{
    AsyncBody, HttpClientWithUrl, HttpRequestExt, Method, Request, Response, StatusCode,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SYSTEM_ID_HEADER_NAME: &str = "x-zed-system-id";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAuthenticatedUserResponse {
    pub user: AuthenticatedUser,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub metrics_id: String,
    pub username: String,
    pub avatar_url: String,
    pub name: Option<String>,
}

struct Credentials {
    user_id: u32,
    access_token: String,
}

#[derive(Debug, Error)]
pub enum AccountApiError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("not signed in")]
    NotSignedIn,
    #[error("connection to {host} failed")]
    ConnectionFailed {
        host: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("{host} returned {status}")]
    ServerError {
        host: String,
        status: StatusCode,
        body: String,
    },
    #[error("invalid response")]
    InvalidResponse(#[source] anyhow::Error),
    #[error("failed to build request")]
    RequestBuildFailed(#[source] anyhow::Error),
}

pub struct AccountClient {
    credentials: RwLock<Option<Credentials>>,
    http_client: Arc<HttpClientWithUrl>,
}

impl AccountClient {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            credentials: RwLock::new(None),
            http_client,
        }
    }

    pub fn set_credentials(&self, user_id: u32, access_token: String) {
        *self.credentials.write() = Some(Credentials {
            user_id,
            access_token,
        });
    }

    pub fn clear_credentials(&self) {
        *self.credentials.write() = None;
    }

    pub async fn get_authenticated_user(
        &self,
        system_id: Option<String>,
    ) -> Result<GetAuthenticatedUserResponse, AccountApiError> {
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me")
                    .map_err(AccountApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(SYSTEM_ID_HEADER_NAME, system_id)
            });

        let mut response = self
            .send_authenticated_request(request_builder, AsyncBody::default())
            .await?;
        let body = Self::read_response_body(&mut response).await?;
        serde_json::from_str(&body).map_err(|error| AccountApiError::InvalidResponse(error.into()))
    }

    async fn send_authenticated_request(
        &self,
        request_builder: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<Response<AsyncBody>, AccountApiError> {
        let request = {
            let credentials = self.credentials.read();
            let credentials = credentials.as_ref().ok_or(AccountApiError::NotSignedIn)?;
            build_request(request_builder, body, credentials)
                .map_err(AccountApiError::RequestBuildFailed)?
        };

        let host = self
            .http_client
            .build_zed_cloud_url("/")
            .ok()
            .and_then(|url| url.host_str().map(String::from))
            .unwrap_or_else(|| "cloud.zed.dev".into());
        let mut response = self.http_client.send(request).await.map_err(|source| {
            AccountApiError::ConnectionFailed {
                host: host.clone(),
                source,
            }
        })?;

        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        if status == StatusCode::UNAUTHORIZED {
            return Err(AccountApiError::Unauthorized);
        }

        let body = Self::read_response_body(&mut response)
            .await
            .unwrap_or_else(|error| format!("failed to read response body: {error}"));
        Err(AccountApiError::ServerError { host, status, body })
    }

    async fn read_response_body(
        response: &mut Response<AsyncBody>,
    ) -> Result<String, AccountApiError> {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| AccountApiError::InvalidResponse(error.into()))?;
        Ok(body)
    }

    pub async fn validate_credentials(&self, user_id: u32, access_token: &str) -> Result<bool> {
        let request = build_request(
            Request::builder().method(Method::GET).uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me")?
                    .as_ref(),
            ),
            AsyncBody::default(),
            &Credentials {
                user_id,
                access_token: access_token.into(),
            },
        )?;

        let mut response = self.http_client.send(request).await?;
        if response.status().is_success() {
            return Ok(true);
        }
        if response.status() == StatusCode::UNAUTHORIZED {
            return Ok(false);
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Err(anyhow!(
            "Failed to get authenticated user.\nStatus: {:?}\nBody: {body}",
            response.status()
        ))
    }
}

fn build_request(
    request: request::Builder,
    body: impl Into<AsyncBody>,
    credentials: &Credentials,
) -> Result<Request<AsyncBody>> {
    // 协作服务沿用用户 ID 与访问令牌组合的认证头。
    Ok(request
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("{} {}", credentials.user_id, credentials.access_token),
        )
        .body(body.into())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticated_request_contains_authorization_header() -> Result<()> {
        let request = build_request(
            Request::builder().uri("http://127.0.0.1/client/users/me"),
            AsyncBody::default(),
            &Credentials {
                user_id: 123,
                access_token: "token".into(),
            },
        )?;

        assert_eq!(
            request
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok()),
            Some("123 token")
        );
        Ok(())
    }
}
