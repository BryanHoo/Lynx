#![allow(non_snake_case)]

pub mod error;

pub use error::*;
pub use prost::{DecodeError, Message};
use std::{
    cmp,
    fmt::Debug,
    iter, mem,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// 兼容尚在迁移中的领域模块导入形式，内容均为本地数据模型。
pub mod proto {
    pub use crate::*;
}

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

impl From<Timestamp> for SystemTime {
    fn from(val: Timestamp) -> Self {
        UNIX_EPOCH
            .checked_add(Duration::new(val.seconds, val.nanos))
            .unwrap()
    }
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
        Self {
            seconds: duration.as_secs(),
            nanos: duration.subsec_nanos(),
        }
    }
}

impl From<u128> for Nonce {
    fn from(nonce: u128) -> Self {
        let upper_half = (nonce >> 64) as u64;
        let lower_half = nonce as u64;
        Self {
            upper_half,
            lower_half,
        }
    }
}

impl From<Nonce> for u128 {
    fn from(nonce: Nonce) -> Self {
        let upper_half = (nonce.upper_half as u128) << 64;
        let lower_half = nonce.lower_half as u128;
        upper_half | lower_half
    }
}

#[cfg(any(test, feature = "test-support"))]
pub const MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE: usize = 2;
#[cfg(not(any(test, feature = "test-support")))]
pub const MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE: usize = 256;

pub fn split_worktree_update(mut message: UpdateWorktree) -> impl Iterator<Item = UpdateWorktree> {
    let mut done = false;

    iter::from_fn(move || {
        if done {
            return None;
        }

        let updated_entries_chunk_size = cmp::min(
            message.updated_entries.len(),
            MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE,
        );
        let updated_entries: Vec<_> = message
            .updated_entries
            .drain(..updated_entries_chunk_size)
            .collect();

        let removed_entries_chunk_size = cmp::min(
            message.removed_entries.len(),
            MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE,
        );
        let removed_entries = message
            .removed_entries
            .drain(..removed_entries_chunk_size)
            .collect();

        let mut updated_repositories = Vec::new();
        let mut limit = MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE;
        while let Some(repo) = message.updated_repositories.first_mut() {
            let updated_statuses_limit = cmp::min(repo.updated_statuses.len(), limit);
            let removed_statuses_limit = cmp::min(repo.removed_statuses.len(), limit);

            updated_repositories.push(RepositoryEntry {
                repository_id: repo.repository_id,
                branch_summary: repo.branch_summary.clone(),
                updated_statuses: repo
                    .updated_statuses
                    .drain(..updated_statuses_limit)
                    .collect(),
                removed_statuses: repo
                    .removed_statuses
                    .drain(..removed_statuses_limit)
                    .collect(),
                current_merge_conflicts: repo.current_merge_conflicts.clone(),
            });
            if repo.removed_statuses.is_empty() && repo.updated_statuses.is_empty() {
                message.updated_repositories.remove(0);
            }
            limit = limit.saturating_sub(removed_statuses_limit + updated_statuses_limit);
            if limit == 0 {
                break;
            }
        }

        done = message.updated_entries.is_empty()
            && message.removed_entries.is_empty()
            && message.updated_repositories.is_empty();

        let removed_repositories = if done {
            mem::take(&mut message.removed_repositories)
        } else {
            Default::default()
        };

        Some(UpdateWorktree {
            project_id: message.project_id,
            worktree_id: message.worktree_id,
            root_name: message.root_name.clone(),
            abs_path: message.abs_path.clone(),
            root_repo_common_dir: message.root_repo_common_dir.clone(),
            root_repo_is_linked_worktree: message.root_repo_is_linked_worktree,
            updated_entries,
            removed_entries,
            scan_id: message.scan_id,
            is_last_update: done && message.is_last_update,
            updated_repositories,
            removed_repositories,
        })
    })
}

pub fn split_repository_update(
    mut update: UpdateRepository,
) -> impl Iterator<Item = UpdateRepository> {
    let mut updated_statuses_iter = mem::take(&mut update.updated_statuses).into_iter().fuse();
    let mut removed_statuses_iter = mem::take(&mut update.removed_statuses).into_iter().fuse();
    let branch_list = mem::take(&mut update.branch_list);
    let branch_list_error = update.branch_list_error.take();
    std::iter::from_fn({
        let update = update.clone();
        move || {
            let updated_statuses = updated_statuses_iter
                .by_ref()
                .take(MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE)
                .collect::<Vec<_>>();
            let removed_statuses = removed_statuses_iter
                .by_ref()
                .take(MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE)
                .collect::<Vec<_>>();
            if updated_statuses.is_empty() && removed_statuses.is_empty() {
                return None;
            }
            Some(UpdateRepository {
                updated_statuses,
                removed_statuses,
                branch_list: Vec::new(),
                branch_list_error: None,
                is_last_update: false,
                ..update.clone()
            })
        }
    })
    .chain([UpdateRepository {
        updated_statuses: Vec::new(),
        removed_statuses: Vec::new(),
        branch_list,
        branch_list_error,
        is_last_update: true,
        ..update
    }])
}

impl LspQuery {
    pub fn query_name_and_write_permissions(&self) -> (&str, bool) {
        match self.request {
            Some(lsp_query::Request::GetHover(_)) => ("GetHover", false),
            Some(lsp_query::Request::GetCodeActions(_)) => ("GetCodeActions", true),
            Some(lsp_query::Request::GetSignatureHelp(_)) => ("GetSignatureHelp", false),
            Some(lsp_query::Request::GetCodeLens(_)) => ("GetCodeLens", true),
            Some(lsp_query::Request::GetDocumentDiagnostics(_)) => {
                ("GetDocumentDiagnostics", false)
            }
            Some(lsp_query::Request::GetDefinition(_)) => ("GetDefinition", false),
            Some(lsp_query::Request::GetEditPredictionDefinition(_)) => {
                ("GetEditPredictionDefinition", false)
            }
            Some(lsp_query::Request::GetEditPredictionTypeDefinition(_)) => {
                ("GetEditPredictionTypeDefinition", false)
            }
            Some(lsp_query::Request::GetDeclaration(_)) => ("GetDeclaration", false),
            Some(lsp_query::Request::GetTypeDefinition(_)) => ("GetTypeDefinition", false),
            Some(lsp_query::Request::GetImplementation(_)) => ("GetImplementation", false),
            Some(lsp_query::Request::GetReferences(_)) => ("GetReferences", false),
            Some(lsp_query::Request::GetDocumentColor(_)) => ("GetDocumentColor", false),
            Some(lsp_query::Request::GetFoldingRanges(_)) => ("GetFoldingRanges", false),
            Some(lsp_query::Request::GetDocumentSymbols(_)) => ("GetDocumentSymbols", false),
            Some(lsp_query::Request::GetDocumentLinks(_)) => ("GetDocumentLinks", false),
            Some(lsp_query::Request::InlayHints(_)) => ("InlayHints", false),
            Some(lsp_query::Request::SemanticTokens(_)) => ("SemanticTokens", false),
            Some(lsp_query::Request::PrepareCallHierarchy(_)) => ("PrepareCallHierarchy", false),
            Some(lsp_query::Request::GetIncomingCalls(_)) => ("GetIncomingCalls", false),
            Some(lsp_query::Request::GetOutgoingCalls(_)) => ("GetOutgoingCalls", false),
            None => ("<unknown>", true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_repository_update_keeps_branch_list_on_final_chunk() {
        let update = UpdateRepository {
            updated_statuses: vec![
                StatusEntry::default(),
                StatusEntry::default(),
                StatusEntry::default(),
            ],
            branch_list: vec![Branch {
                ref_name: "refs/heads/main".into(),
                ..Default::default()
            }],
            branch_list_error: Some("partial branch scan".into()),
            ..Default::default()
        };

        let chunks = split_repository_update(update).collect::<Vec<_>>();

        assert_eq!(chunks.len(), 3);
        assert!(chunks[0].branch_list.is_empty());
        assert!(chunks[1].branch_list.is_empty());
        assert_eq!(chunks[2].branch_list.len(), 1);
        assert_eq!(chunks[2].branch_list[0].ref_name, "refs/heads/main");
        assert_eq!(chunks[0].branch_list_error, None);
        assert_eq!(chunks[1].branch_list_error, None);
        assert_eq!(
            chunks[2].branch_list_error.as_deref(),
            Some("partial branch scan")
        );
        assert!(!chunks[0].is_last_update);
        assert!(!chunks[1].is_last_update);
        assert!(chunks[2].is_last_update);
    }
}
