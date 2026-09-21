use super::{Client, Status, proto};
use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::{StreamExt, channel::mpsc};
use gpui::{App, Context, SharedString, SharedUri, Task, TaskExt, WeakEntity};
use postage::{sink::Sink, watch};
use rpc::proto::{RequestMessage, UsersResponse};
use std::sync::{Arc, Weak};
use util::ResultExt;

pub type LegacyUserId = u64;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct ChannelId(pub u64);

impl std::fmt::Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ProjectId(pub u64);

impl ProjectId {
    pub fn to_proto(self) -> u64 {
        self.0
    }
}

#[derive(Default, Debug)]
pub struct User {
    pub legacy_id: LegacyUserId,
    pub username: SharedString,
    pub avatar_uri: SharedUri,
    pub name: Option<String>,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.username.cmp(&other.username)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.legacy_id == other.legacy_id && self.username == other.username
    }
}

impl Eq for User {}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    client: Weak<Client>,
    _maintain_current_user: Task<Result<()>>,
    _handle_sign_out: Task<()>,
    weak_self: WeakEntity<Self>,
}

impl UserStore {
    pub fn new(client: Arc<Client>, cx: &Context<Self>) -> Self {
        let (mut current_user_tx, current_user_rx) = watch::channel();
        let (sign_out_tx, mut sign_out_rx) = mpsc::unbounded();
        client.sign_out_tx.lock().replace(sign_out_tx);
        Self {
            users: Default::default(),
            current_user: current_user_rx,
            client: Arc::downgrade(&client),
            _maintain_current_user: cx.spawn(async move |this, cx| {
                let mut status = client.status();
                let weak = Arc::downgrade(&client);
                drop(client);
                while let Some(status) = status.next().await {
                    // If the client is dropped, the app is shutting down.
                    let Some(client) = weak.upgrade() else {
                        return Ok(());
                    };
                    match status {
                        Status::Authenticated
                        | Status::Reauthenticated
                        | Status::Connected { .. } => {
                            if let Some(user_id) = client.user_id() {
                                let response = client
                                    .account_client()
                                    .get_authenticated_user()
                                    .await
                                    .log_err();

                                let current_user = if let Some(response) = response {
                                    let user = Arc::new(User {
                                        legacy_id: user_id,
                                        username: response.user.username.clone().into(),
                                        avatar_uri: response.user.avatar_url.clone().into(),
                                        name: response.user.name.clone(),
                                    });

                                    Some(user)
                                } else {
                                    None
                                };
                                current_user_tx.send(current_user.clone()).await.ok();

                                cx.update(|cx| {
                                    if let Some(user) = current_user {
                                        this.update(cx, |this, _cx| {
                                            this.users.insert(user_id, user);
                                        })
                                    } else {
                                        anyhow::Ok(())
                                    }
                                })?;

                                this.update(cx, |_, cx| cx.notify())?;
                            }
                        }
                        Status::SignedOut => {
                            current_user_tx.send(None).await.ok();
                            this.update(cx, |_, cx| cx.notify())?;
                        }
                        Status::ConnectionLost => {
                            this.update(cx, |_, cx| cx.notify())?;
                        }
                        _ => {}
                    }
                }
                Ok(())
            }),
            _handle_sign_out: cx.spawn(async move |this, cx| {
                while let Some(()) = sign_out_rx.next().await {
                    let Some(client) = this
                        .read_with(cx, |this, _cx| this.client.upgrade())
                        .ok()
                        .flatten()
                    else {
                        break;
                    };

                    client.sign_out(cx).await;
                }
            }),
            weak_self: cx.weak_entity(),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn clear_cache(&mut self) {
        self.users.clear();
    }

    pub fn get_users(
        &self,
        user_ids: Vec<u64>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let mut user_ids_to_fetch = user_ids.clone();
        user_ids_to_fetch.retain(|id| !self.users.contains_key(id));

        cx.spawn(async move |this, cx| {
            if !user_ids_to_fetch.is_empty() {
                this.update(cx, |this, cx| {
                    this.load_users(
                        proto::GetUsers {
                            user_ids: user_ids_to_fetch,
                        },
                        cx,
                    )
                })?
                .await?;
            }

            this.read_with(cx, |this, _| {
                user_ids
                    .iter()
                    .map(|user_id| {
                        this.users
                            .get(user_id)
                            .cloned()
                            .with_context(|| format!("user {user_id} not found"))
                    })
                    .collect()
            })?
        })
    }

    pub fn fuzzy_search_users(
        &self,
        query: String,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        self.load_users(proto::FuzzySearchUsers { query }, cx)
    }

    pub fn get_cached_user(&self, user_id: u64) -> Option<Arc<User>> {
        self.users.get(&user_id).cloned()
    }

    pub fn get_user_optimistic(&self, user_id: u64, cx: &Context<Self>) -> Option<Arc<User>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return Some(user);
        }

        self.get_user(user_id, cx).detach_and_log_err(cx);
        None
    }

    pub fn get_user(&self, user_id: u64, cx: &Context<Self>) -> Task<Result<Arc<User>>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return Task::ready(Ok(user));
        }

        let load_users = self.get_users(vec![user_id], cx);
        cx.spawn(async move |this, cx| {
            load_users.await?;
            this.read_with(cx, |this, _| {
                this.users
                    .get(&user_id)
                    .cloned()
                    .context("server responded with no users")
            })?
        })
    }

    pub fn current_user(&self) -> Option<Arc<User>> {
        self.current_user.borrow().clone()
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
    }

    fn load_users(
        &self,
        request: impl RequestMessage<Response = UsersResponse>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let client = self.client.clone();
        cx.spawn(async move |this, cx| {
            if let Some(rpc) = client.upgrade() {
                let response = rpc.request(request).await.context("error loading users")?;
                let users = response.users;

                this.update(cx, |this, _| this.insert(users))
            } else {
                Ok(Vec::new())
            }
        })
    }

    pub fn insert(&mut self, users: Vec<proto::User>) -> Vec<Arc<User>> {
        let mut ret = Vec::with_capacity(users.len());
        for user in users {
            let user = User::new(user);
            self.users.insert(user.legacy_id, user.clone());
            ret.push(user)
        }
        ret
    }

    pub fn participant_names(
        &self,
        user_ids: impl Iterator<Item = u64>,
        cx: &App,
    ) -> HashMap<u64, SharedString> {
        let mut ret = HashMap::default();
        let mut missing_user_ids = Vec::new();
        for id in user_ids {
            if let Some(username) = self.get_cached_user(id).map(|u| u.username.clone()) {
                ret.insert(id, username);
            } else {
                missing_user_ids.push(id)
            }
        }
        if !missing_user_ids.is_empty() {
            let this = self.weak_self.clone();
            cx.spawn(async move |cx| {
                this.update(cx, |this, cx| this.get_users(missing_user_ids, cx))?
                    .await
            })
            .detach_and_log_err(cx);
        }
        ret
    }
}

impl User {
    fn new(message: proto::User) -> Arc<Self> {
        Arc::new(User {
            legacy_id: message.id,
            username: message.username.into(),
            avatar_uri: message.avatar_url.into(),
            name: message.name,
        })
    }
}
