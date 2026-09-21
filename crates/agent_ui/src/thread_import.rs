use std::time::Duration;

use crate::RemoteConnectionOptions;
use acp_thread::AgentSessionListRequest;
use agent::ThreadStore;
use agent_client_protocol::schema::v1 as acp;
use chrono::Utc;
use collections::{HashMap, HashSet};
use db::kvp::Dismissable;
use fs::Fs;
use futures::FutureExt as _;
use gpui::{
    Animation, AnimationExt as _, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, MouseDownEvent, Render, SharedString, Task, TaskExt, WeakEntity, Window,
    pulsating_between,
};
use itertools::Itertools as _;
use notifications::status_toast::StatusToast;
use project::{AgentId, AgentRegistryStore, AgentServerStore};
use ui::{
    Checkbox, CommonAnimationExt, KeyBinding, ListItem, ListItemSpacing, Modal, ModalFooter,
    ModalHeader, Section, Tooltip, prelude::*,
};
use util::ResultExt;
use workspace::{ModalView, MultiWorkspace, Workspace};

use crate::{
    Agent, AgentPanel,
    agent_connection_store::AgentConnectionStore,
    thread_metadata_store::{ThreadId, ThreadMetadata, ThreadMetadataStore, WorktreePaths},
};

pub struct AcpThreadImportOnboarding;

impl AcpThreadImportOnboarding {
    pub fn dismissed(cx: &App) -> bool {
        <Self as Dismissable>::dismissed(cx)
    }

    pub fn dismiss(cx: &mut App) {
        <Self as Dismissable>::set_dismissed(true, cx);
    }
}

impl Dismissable for AcpThreadImportOnboarding {
    const KEY: &'static str = "dismissed-acp-thread-import";
}

#[derive(Clone)]
struct AgentEntry {
    agent_id: AgentId,
    display_name: SharedString,
    icon_path: Option<SharedString>,
}

#[derive(Clone)]
enum AgentImportStatus {
    Loading,
    Ready { importable_count: usize },
    Unsupported,
    Error(SharedString),
}

impl AgentImportStatus {
    fn is_selectable(&self) -> bool {
        matches!(self, Self::Ready { importable_count } if *importable_count > 0)
    }

    fn tooltip_text(&self) -> Option<SharedString> {
        match self {
            Self::Loading => Some("Fetching Sessions…".into()),
            Self::Ready { .. } => None,
            Self::Unsupported => Some("Importing threads from this agent is not possible as it doesn't support ACP's session/list capability.".into()),
            Self::Error(error) => Some(format!("Failed to fetch sessions: {error}").into()),
        }
    }
}

pub struct ThreadImportModal {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    multi_workspace: WeakEntity<MultiWorkspace>,
    agent_entries: Vec<AgentEntry>,
    unchecked_agents: HashSet<AgentId>,
    agent_import_statuses: HashMap<AgentId, AgentImportStatus>,
    sessions_by_agent: Vec<SessionByAgent>,
    selected_index: Option<usize>,
    is_fetching_sessions: bool,
    is_importing: bool,
    last_error: Option<SharedString>,
}

impl ThreadImportModal {
    pub fn new(
        agent_server_store: Entity<AgentServerStore>,
        agent_registry_store: Entity<AgentRegistryStore>,
        workspace: WeakEntity<Workspace>,
        multi_workspace: WeakEntity<MultiWorkspace>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        AcpThreadImportOnboarding::dismiss(cx);

        let agent_entries = agent_server_store
            .read(cx)
            .external_agents()
            .map(|agent_id| {
                let display_name = agent_server_store
                    .read(cx)
                    .agent_display_name(agent_id)
                    .or_else(|| {
                        agent_registry_store
                            .read(cx)
                            .agent(agent_id)
                            .map(|agent| agent.name().clone())
                    })
                    .unwrap_or_else(|| agent_id.0.clone());
                let icon_path = agent_server_store
                    .read(cx)
                    .agent_icon(agent_id)
                    .or_else(|| {
                        agent_registry_store
                            .read(cx)
                            .agent(agent_id)
                            .and_then(|agent| agent.icon_path().cloned())
                    });

                AgentEntry {
                    agent_id: agent_id.clone(),
                    display_name,
                    icon_path,
                }
            })
            .sorted_unstable_by_key(|entry| entry.display_name.to_lowercase())
            .collect::<Vec<_>>();

        let this = Self {
            focus_handle: cx.focus_handle(),
            workspace,
            multi_workspace,
            agent_entries,
            unchecked_agents: HashSet::default(),
            agent_import_statuses: HashMap::default(),
            sessions_by_agent: Vec::new(),
            selected_index: None,
            is_fetching_sessions: false,
            is_importing: false,
            last_error: None,
        };
        cx.spawn(async move |this, cx| this.update(cx, |this, cx| this.fetch_sessions(cx)))
            .detach_and_log_err(cx);
        this
    }

    fn agent_ids(&self) -> Vec<AgentId> {
        self.agent_entries
            .iter()
            .map(|entry| entry.agent_id.clone())
            .collect()
    }

    fn fetch_sessions(&mut self, cx: &mut Context<Self>) {
        if self.agent_entries.is_empty() {
            return;
        }

        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            self.mark_all_agents_failed("Could not find workspace to import from.");
            return;
        };

        let stores = resolve_agent_connection_stores(&multi_workspace, cx);
        if stores.is_empty() {
            log::error!("Did not find any workspaces to import from");
            self.mark_all_agents_failed("Did not find any workspaces to import from.");
            return;
        }

        self.is_fetching_sessions = true;
        self.last_error = None;
        self.sessions_by_agent.clear();
        self.agent_import_statuses = self
            .agent_ids()
            .into_iter()
            .map(|agent_id| (agent_id, AgentImportStatus::Loading))
            .collect();

        let existing_sessions: HashSet<acp::SessionId> = ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .filter_map(|metadata| metadata.session_id.clone())
            .collect();

        for agent_id in self.agent_ids() {
            let task =
                fetch_sessions_for_agent(agent_id, existing_sessions.clone(), stores.clone(), cx);
            cx.spawn(async move |this, cx| {
                let result = task.await;
                this.update(cx, |this, cx| {
                    let AgentSessionFetchResult {
                        agent_id,
                        sessions_by_agent,
                        status,
                    } = result;
                    this.sessions_by_agent
                        .retain(|sessions| sessions.agent_id != agent_id);
                    this.sessions_by_agent.extend(sessions_by_agent);
                    this.agent_import_statuses.insert(agent_id, status);
                    this.is_fetching_sessions = this.has_loading_agents();
                    cx.notify();
                })
            })
            .detach_and_log_err(cx);
        }
    }

    fn mark_all_agents_failed(&mut self, message: impl Into<SharedString>) {
        let message = message.into();
        self.is_fetching_sessions = false;
        self.sessions_by_agent.clear();
        self.last_error = Some(message.clone());
        self.agent_import_statuses = self
            .agent_ids()
            .into_iter()
            .map(|agent_id| (agent_id, AgentImportStatus::Error(message.clone())))
            .collect();
    }

    fn agent_is_selectable(&self, agent_id: &AgentId) -> bool {
        self.agent_import_statuses
            .get(agent_id)
            .map_or(false, AgentImportStatus::is_selectable)
    }

    fn has_checked_selectable_agent(&self) -> bool {
        self.agent_entries.iter().any(|entry| {
            self.agent_is_selectable(&entry.agent_id)
                && !self.unchecked_agents.contains(&entry.agent_id)
        })
    }

    fn has_loading_agents(&self) -> bool {
        self.agent_import_statuses
            .values()
            .any(|status| matches!(status, AgentImportStatus::Loading))
    }

    fn toggle_agent_checked(&mut self, agent_id: AgentId, cx: &mut Context<Self>) {
        if self.is_importing || !self.agent_is_selectable(&agent_id) {
            return;
        }

        if self.unchecked_agents.contains(&agent_id) {
            self.unchecked_agents.remove(&agent_id);
        } else {
            self.unchecked_agents.insert(agent_id);
        }
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        if self.agent_entries.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(ix) if ix + 1 >= self.agent_entries.len() => 0,
            Some(ix) => ix + 1,
            None => 0,
        });
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.agent_entries.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(0) => self.agent_entries.len() - 1,
            Some(ix) => ix - 1,
            None => self.agent_entries.len() - 1,
        });
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected_index {
            if let Some(entry) = self.agent_entries.get(ix) {
                self.toggle_agent_checked(entry.agent_id.clone(), cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn import_threads(
        &mut self,
        _: &menu::SecondaryConfirm,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_importing || !self.has_checked_selectable_agent() {
            return;
        }

        self.is_importing = true;
        self.last_error = None;

        let existing_sessions: HashSet<acp::SessionId> = ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .filter_map(|metadata| metadata.session_id.clone())
            .collect();

        let selected_sessions_by_agent = self
            .sessions_by_agent
            .iter()
            .filter(|sessions| {
                self.agent_is_selectable(&sessions.agent_id)
                    && !self.unchecked_agents.contains(&sessions.agent_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        let threads = collect_importable_threads(selected_sessions_by_agent, existing_sessions);
        let imported_count = threads.len();

        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save_all(threads, cx));

        self.is_importing = false;
        self.show_imported_threads_toast(imported_count, cx);
        cx.emit(DismissEvent);
    }

    fn show_imported_threads_toast(&self, imported_count: usize, cx: &mut App) {
        let status_toast = if imported_count == 0 {
            StatusToast::new("No threads found to import.", cx, |this, _cx| {
                this.icon(
                    Icon::new(IconName::Info)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .dismiss_button(true)
            })
        } else {
            let message = if imported_count == 1 {
                "Imported 1 thread.".to_string()
            } else {
                format!("Imported {imported_count} threads.")
            };
            StatusToast::new(message, cx, |this, _cx| {
                this.icon(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Success),
                )
                .dismiss_button(true)
            })
        };

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_status_toast(status_toast, cx);
            })
            .log_err();
    }
}

impl EventEmitter<DismissEvent> for ThreadImportModal {}

impl Focusable for ThreadImportModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ThreadImportModal {}

impl Render for ThreadImportModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_agents = !self.agent_entries.is_empty();
        let disabled_import_thread =
            self.is_importing || !has_agents || !self.has_checked_selectable_agent();

        let agent_rows = self
            .agent_entries
            .iter()
            .enumerate()
            .map(|(ix, entry)| {
                let status = self
                    .agent_import_statuses
                    .get(&entry.agent_id)
                    .cloned()
                    .unwrap_or(AgentImportStatus::Loading);
                let is_selectable = status.is_selectable();
                let is_checked = is_selectable && !self.unchecked_agents.contains(&entry.agent_id);
                let is_focused = self.selected_index == Some(ix);
                let row_disabled = self.is_importing || !is_selectable;
                let checkbox_state = if is_checked {
                    ToggleState::Selected
                } else {
                    ToggleState::Unselected
                };
                let end_slot = match &status {
                    AgentImportStatus::Loading
                    | AgentImportStatus::Unsupported
                    | AgentImportStatus::Error(_) => {
                        Checkbox::new(("thread-import-agent-checkbox", ix), checkbox_state)
                            .disabled(true)
                            .into_any_element()
                    }
                    AgentImportStatus::Ready { .. } => {
                        Checkbox::new(("thread-import-agent-checkbox", ix), checkbox_state)
                            .disabled(row_disabled)
                            .into_any_element()
                    }
                };

                let is_loading = matches!(status, AgentImportStatus::Loading);

                let icon_color = if is_checked {
                    Color::Muted
                } else {
                    Color::Disabled
                };

                let item = h_flex()
                    .w_full()
                    .gap_2()
                    .child(if let Some(icon_path) = entry.icon_path.clone() {
                        Icon::from_external_svg(icon_path)
                            .color(icon_color)
                            .size(IconSize::Small)
                    } else {
                        Icon::new(IconName::Sparkle)
                            .color(icon_color)
                            .size(IconSize::Small)
                    })
                    .child(
                        Label::new(entry.display_name.clone())
                            .when(!is_checked, |s| s.color(Color::Disabled)),
                    )
                    .map(|this| match status {
                        AgentImportStatus::Loading => this,
                        AgentImportStatus::Ready {
                            importable_count: count,
                        } => {
                            let label: SharedString = if count == 0 {
                                "No threads".into()
                            } else {
                                format!("{} threads", count).into()
                            };
                            this.child(Label::new(label).size(LabelSize::Small).color(Color::Muted))
                        }
                        AgentImportStatus::Unsupported => this.child(
                            Icon::new(IconName::Warning)
                                .color(Color::Warning)
                                .size(IconSize::Small),
                        ),
                        AgentImportStatus::Error(_) => this.child(
                            Icon::new(IconName::XCircle)
                                .color(Color::Error)
                                .size(IconSize::Small),
                        ),
                    });

                let item = if is_loading {
                    item.with_animation(
                        "pulsating-icon",
                        Animation::new(Duration::from_secs(1))
                            .repeat()
                            .with_easing(pulsating_between(0.2, 0.6)),
                        |icon, delta| icon.opacity(delta),
                    )
                    .into_any_element()
                } else {
                    item.into_any_element()
                };

                ListItem::new(("thread-import-agent", ix))
                    .rounded()
                    .spacing(ListItemSpacing::Sparse)
                    .focused(is_focused)
                    .disabled(row_disabled)
                    .child(item)
                    .end_slot(end_slot)
                    .when_some(status.tooltip_text(), |this, tooltip| {
                        this.tooltip(Tooltip::text(tooltip))
                    })
                    .on_click({
                        let agent_id = entry.agent_id.clone();
                        cx.listener(move |this, _event, _window, cx| {
                            this.toggle_agent_checked(agent_id.clone(), cx);
                        })
                    })
            })
            .collect::<Vec<_>>();

        v_flex()
            .id("thread-import-modal")
            .key_context("ThreadImportModal")
            .w(rems(34.))
            .elevation_3(cx)
            .overflow_hidden()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::import_threads))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                this.focus_handle.focus(window, cx);
            }))
            .child(
                Modal::new("import-threads", None)
                    .header(
                        ModalHeader::new()
                            .headline("Import External Agent Threads")
                            .description(
                                "Import threads from agents like Claude Agent, Codex, and more, whether started in Lynx or another client. \
                                Choose which agents to include, and their threads will appear in your thread history."
                            )
                            .show_dismiss_button(true),

                    )
                    .section(
                        Section::new().child(
                            v_flex()
                                .id("thread-import-agent-list")
                                .max_h(rems_from_px(320_f32))
                                .pb_1()
                                .overflow_y_scroll()
                                .when(has_agents, |this| this.children(agent_rows))
                                .when(!has_agents, |this| {
                                    this.child(
                                        Label::new("No external agents available.")
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    )
                                }),
                        ),
                    )
                    .footer(
                        ModalFooter::new()
                            .when(self.is_fetching_sessions, |this| {
                                this.start_slot(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Icon::new(IconName::LoadCircle)
                                                .size(IconSize::Small)
                                                .color(Color::Muted)
                                                .with_rotate_animation(3),
                                        )
                                        .child(Label::new("Fetching Agent Threads…")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted))

                                )
                            })
                            .when_some(self.last_error.clone(), |this, error| {
                                this.start_slot(
                                    Label::new(error)
                                        .size(LabelSize::Small)
                                        .color(Color::Error)
                                        .truncate(),
                                )
                            })
                            .end_slot(
                                Button::new("import-threads", "Import Threads")
                                    .loading(self.is_importing)
                                    .disabled(disabled_import_thread)
                                    .key_binding(
                                        KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                            .map(|kb| kb.size(rems_from_px(12_f32))),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.import_threads(&menu::SecondaryConfirm, window, cx);
                                    })),
                            ),
                    ),
            )
    }
}

fn resolve_agent_connection_stores(
    multi_workspace: &Entity<MultiWorkspace>,
    cx: &App,
) -> Vec<Entity<AgentConnectionStore>> {
    let mut stores = Vec::new();
    let mut included_local_store = false;

    for workspace in multi_workspace.read(cx).workspaces() {
        let workspace = workspace.read(cx);
        let project = workspace.project().read(cx);

        // We only want to include scores from one local workspace, since we
        // know that they live on the same machine
        let include_store = if project.is_remote() {
            true
        } else if project.is_local() && !included_local_store {
            included_local_store = true;
            true
        } else {
            false
        };

        if !include_store {
            continue;
        }

        if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
            stores.push(panel.read(cx).connection_store().clone());
        }
    }

    stores
}

struct AgentSessionFetchResult {
    agent_id: AgentId,
    sessions_by_agent: Vec<SessionByAgent>,
    status: AgentImportStatus,
}

#[derive(Default)]
struct AgentSessionFetchStats {
    supported_attempt_count: usize,
    successful_attempt_count: usize,
    unsupported_attempt_count: usize,
    errors: Vec<SharedString>,
}

fn fetch_sessions_for_agent(
    agent_id: AgentId,
    existing_sessions: HashSet<acp::SessionId>,
    stores: Vec<Entity<AgentConnectionStore>>,
    cx: &mut App,
) -> Task<AgentSessionFetchResult> {
    let mut wait_for_connection_tasks = Vec::new();

    for store in stores {
        let remote_connection = None;
        let agent = Agent::from(agent_id.clone());
        let server = agent.server(<dyn Fs>::global(cx), ThreadStore::global(cx));
        let entry = store.update(cx, |store, cx| store.request_connection(agent, server, cx));

        wait_for_connection_tasks.push(entry.read(cx).wait_for_connection().map({
            let agent_id = agent_id.clone();
            let remote_connection = remote_connection.clone();
            move |result| (agent_id, remote_connection, result)
        }));
    }

    cx.spawn(async move |cx| {
        let mut stats = AgentSessionFetchStats::default();
        let results = futures::future::join_all(wait_for_connection_tasks).await;

        let mut page_tasks = Vec::new();
        for (agent_id, remote_connection, result) in results {
            let state = match result {
                Ok(state) => state,
                Err(error) => {
                    log::warn!("Failed to connect to {agent_id} to list sessions: {error}");
                    stats.errors.push(error.to_string().into());
                    continue;
                }
            };

            let Some(list) = cx.update(|cx| state.connection.session_list(cx)) else {
                stats.unsupported_attempt_count += 1;
                continue;
            };

            stats.supported_attempt_count += 1;
            page_tasks.push(cx.spawn({
                let list = list.clone();
                let agent_id_for_error = agent_id.clone();
                async move |cx| {
                    (
                        agent_id_for_error,
                        collect_all_sessions(agent_id, remote_connection, list, cx).await,
                    )
                }
            }));
        }

        let mut sessions_by_agent = Vec::new();
        for (agent_id, result) in futures::future::join_all(page_tasks).await {
            match result {
                Ok(sessions) => {
                    stats.successful_attempt_count += 1;
                    sessions_by_agent.push(sessions);
                }
                Err(error) => {
                    log::warn!("Failed to list sessions for {agent_id}: {error}");
                    stats.errors.push(error.to_string().into());
                }
            }
        }

        let importable_counts_by_agent =
            count_importable_threads_by_agent(&sessions_by_agent, &existing_sessions);
        let status = if stats.successful_attempt_count > 0 {
            AgentImportStatus::Ready {
                importable_count: importable_counts_by_agent
                    .get(&agent_id)
                    .copied()
                    .unwrap_or(0),
            }
        } else if stats.supported_attempt_count > 0 {
            AgentImportStatus::Error(
                stats
                    .errors
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Failed to list sessions.".into()),
            )
        } else if stats.unsupported_attempt_count > 0 {
            AgentImportStatus::Unsupported
        } else if let Some(error) = stats.errors.first().cloned() {
            AgentImportStatus::Error(error)
        } else {
            AgentImportStatus::Unsupported
        };

        AgentSessionFetchResult {
            agent_id,
            sessions_by_agent,
            status,
        }
    })
}

async fn collect_all_sessions(
    agent_id: AgentId,
    remote_connection: Option<RemoteConnectionOptions>,
    list: std::rc::Rc<dyn acp_thread::AgentSessionList>,
    cx: &mut gpui::AsyncApp,
) -> anyhow::Result<SessionByAgent> {
    let mut sessions = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = AgentSessionListRequest {
            cursor: cursor.clone(),
            ..Default::default()
        };
        let task = cx.update(|cx| list.list_sessions(request, cx));
        let response = task.await?;
        sessions.extend(response.sessions);
        match response.next_cursor {
            Some(next) if Some(&next) != cursor.as_ref() => cursor = Some(next),
            _ => break,
        }
    }
    Ok(SessionByAgent {
        agent_id,
        remote_connection,
        sessions,
    })
}

#[derive(Clone)]
struct SessionByAgent {
    agent_id: AgentId,
    remote_connection: Option<RemoteConnectionOptions>,
    sessions: Vec<acp_thread::AgentSessionInfo>,
}

fn count_importable_threads_by_agent(
    sessions_by_agent: &[SessionByAgent],
    existing_sessions: &HashSet<acp::SessionId>,
) -> HashMap<AgentId, usize> {
    let mut counts_by_agent = HashMap::default();
    let mut seen_sessions_by_agent = HashMap::<AgentId, HashSet<acp::SessionId>>::default();

    for sessions_for_agent in sessions_by_agent {
        let seen_sessions = seen_sessions_by_agent
            .entry(sessions_for_agent.agent_id.clone())
            .or_insert_with(|| existing_sessions.clone());
        for session in &sessions_for_agent.sessions {
            if !seen_sessions.insert(session.session_id.clone()) {
                continue;
            }
            if session.work_dirs.is_some() {
                *counts_by_agent
                    .entry(sessions_for_agent.agent_id.clone())
                    .or_insert(0) += 1;
            }
        }
    }

    counts_by_agent
}

fn collect_importable_threads(
    sessions_by_agent: Vec<SessionByAgent>,
    mut existing_sessions: HashSet<acp::SessionId>,
) -> Vec<ThreadMetadata> {
    let mut to_insert = Vec::new();
    for SessionByAgent {
        agent_id,
        remote_connection,
        sessions,
    } in sessions_by_agent
    {
        for session in sessions {
            if !existing_sessions.insert(session.session_id.clone()) {
                continue;
            }
            let Some(folder_paths) = session.work_dirs else {
                continue;
            };
            to_insert.push(ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(session.session_id),
                agent_id: agent_id.clone(),
                title: session.title,
                title_override: None,
                updated_at: session.updated_at.unwrap_or_else(|| Utc::now()),
                created_at: session.created_at,
                interacted_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&folder_paths),
                remote_connection: remote_connection.clone(),
                archived: true,
            });
        }
    }
    to_insert
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::AgentSessionInfo;
    use chrono::Utc;
    use std::path::Path;
    use workspace::PathList;

    fn make_session(
        session_id: &str,
        title: Option<&str>,
        work_dirs: Option<PathList>,
        updated_at: Option<chrono::DateTime<Utc>>,
        created_at: Option<chrono::DateTime<Utc>>,
    ) -> AgentSessionInfo {
        AgentSessionInfo {
            session_id: acp::SessionId::new(session_id),
            title: title.map(|t| SharedString::from(t.to_string())),
            work_dirs,
            updated_at,
            created_at,
            meta: None,
        }
    }

    #[test]
    fn test_collect_skips_sessions_already_in_existing_set() {
        let existing = HashSet::from_iter(vec![acp::SessionId::new("existing-1")]);
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session(
                    "existing-1",
                    Some("Already There"),
                    Some(paths.clone()),
                    None,
                    None,
                ),
                make_session("new-1", Some("Brand New"), Some(paths), None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].session_id.as_ref().unwrap().0.as_ref(), "new-1");
        assert_eq!(result[0].display_title(), "Brand New");
    }

    #[test]
    fn test_collect_skips_sessions_without_work_dirs() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session("has-dirs", Some("With Dirs"), Some(paths), None, None),
                make_session("no-dirs", Some("No Dirs"), None, None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].session_id.as_ref().unwrap().0.as_ref(),
            "has-dirs"
        );
    }

    #[test]
    fn test_collect_marks_all_imported_threads_as_archived() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session("s1", Some("Thread 1"), Some(paths.clone()), None, None),
                make_session("s2", Some("Thread 2"), Some(paths), None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|t| t.archived));
    }

    #[test]
    fn test_collect_assigns_correct_agent_id_per_session() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![
            SessionByAgent {
                agent_id: AgentId::new("agent-a"),
                remote_connection: None,
                sessions: vec![make_session(
                    "s1",
                    Some("From A"),
                    Some(paths.clone()),
                    None,
                    None,
                )],
            },
            SessionByAgent {
                agent_id: AgentId::new("agent-b"),
                remote_connection: None,
                sessions: vec![make_session("s2", Some("From B"), Some(paths), None, None)],
            },
        ];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 2);
        let s1 = result
            .iter()
            .find(|t| t.session_id.as_ref().map(|s| s.0.as_ref()) == Some("s1"))
            .unwrap();
        let s2 = result
            .iter()
            .find(|t| t.session_id.as_ref().map(|s| s.0.as_ref()) == Some("s2"))
            .unwrap();
        assert_eq!(s1.agent_id.as_ref(), "agent-a");
        assert_eq!(s2.agent_id.as_ref(), "agent-b");
    }

    #[test]
    fn test_collect_deduplicates_across_agents() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![
            SessionByAgent {
                agent_id: AgentId::new("agent-a"),
                remote_connection: None,
                sessions: vec![make_session(
                    "shared-session",
                    Some("From A"),
                    Some(paths.clone()),
                    None,
                    None,
                )],
            },
            SessionByAgent {
                agent_id: AgentId::new("agent-b"),
                remote_connection: None,
                sessions: vec![make_session(
                    "shared-session",
                    Some("From B"),
                    Some(paths),
                    None,
                    None,
                )],
            },
        ];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].session_id.as_ref().unwrap().0.as_ref(),
            "shared-session"
        );
        assert_eq!(
            result[0].agent_id.as_ref(),
            "agent-a",
            "first agent encountered should win"
        );
    }

    #[test]
    fn test_collect_all_existing_returns_empty() {
        let paths = PathList::new(&[Path::new("/project")]);
        let existing =
            HashSet::from_iter(vec![acp::SessionId::new("s1"), acp::SessionId::new("s2")]);

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session("s1", Some("T1"), Some(paths.clone()), None, None),
                make_session("s2", Some("T2"), Some(paths), None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);
        assert!(result.is_empty());
    }

    #[test]
    fn test_count_importable_threads_by_agent_counts_each_agent_independently() {
        let existing = HashSet::from_iter(vec![acp::SessionId::new("existing")]);
        let paths = PathList::new(&[Path::new("/project")]);
        let sessions_by_agent = vec![
            SessionByAgent {
                agent_id: AgentId::new("agent-a"),
                remote_connection: None,
                sessions: vec![
                    make_session(
                        "existing",
                        Some("Existing"),
                        Some(paths.clone()),
                        None,
                        None,
                    ),
                    make_session("shared", Some("Shared A"), Some(paths.clone()), None, None),
                    make_session("no-dirs", Some("No Dirs"), None, None, None),
                ],
            },
            SessionByAgent {
                agent_id: AgentId::new("agent-b"),
                remote_connection: None,
                sessions: vec![make_session(
                    "shared",
                    Some("Shared B"),
                    Some(paths),
                    None,
                    None,
                )],
            },
        ];

        let counts = count_importable_threads_by_agent(&sessions_by_agent, &existing);

        assert_eq!(counts.get(&AgentId::new("agent-a")), Some(&1));
        assert_eq!(counts.get(&AgentId::new("agent-b")), Some(&1));
    }
}
