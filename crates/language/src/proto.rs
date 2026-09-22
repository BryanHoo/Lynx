//! Handles conversions between language-domain values and local project models.

use crate::{
    CursorShape, Diagnostic, DiagnosticMessage, DiagnosticSourceKind,
    diagnostic_set::DiagnosticEntry,
};
use anyhow::{Context as _, Result};
use clock::ReplicaId;
use gpui::SharedString;
use lsp::{DiagnosticSeverity, LanguageServerId};
use project_models;
use serde_json::Value;
use std::{ops::Range, str::FromStr, sync::Arc};
use text::*;

pub use project_models::{BufferState, File, Operation};

use super::{point_from_lsp, point_to_lsp};

/// Deserializes a `[text::LineEnding]` from the RPC representation.
pub fn deserialize_line_ending(message: project_models::LineEnding) -> text::LineEnding {
    match message {
        project_models::LineEnding::Unix => text::LineEnding::Unix,
        project_models::LineEnding::Windows => text::LineEnding::Windows,
    }
}

/// Serializes a [`text::LineEnding`] to be sent over RPC.
pub fn serialize_line_ending(message: text::LineEnding) -> project_models::LineEnding {
    match message {
        text::LineEnding::Unix => project_models::LineEnding::Unix,
        text::LineEnding::Windows => project_models::LineEnding::Windows,
    }
}

/// Serializes a [`crate::Operation`] to be sent over RPC.
pub fn serialize_operation(operation: &crate::Operation) -> project_models::Operation {
    project_models::Operation {
        variant: Some(match operation {
            crate::Operation::Buffer(text::Operation::Edit(edit)) => {
                project_models::operation::Variant::Edit(serialize_edit_operation(edit))
            }

            crate::Operation::Buffer(text::Operation::Undo(undo)) => {
                project_models::operation::Variant::Undo(project_models::operation::Undo {
                    replica_id: undo.timestamp.replica_id.as_u16() as u32,
                    lamport_timestamp: undo.timestamp.value,
                    version: serialize_version(&undo.version),
                    counts: undo
                        .counts
                        .iter()
                        .map(|(edit_id, count)| project_models::UndoCount {
                            replica_id: edit_id.replica_id.as_u16() as u32,
                            lamport_timestamp: edit_id.value,
                            count: *count,
                        })
                        .collect(),
                })
            }

            crate::Operation::UpdateSelections {
                selections,
                line_mode,
                lamport_timestamp,
                cursor_shape,
            } => project_models::operation::Variant::UpdateSelections(
                project_models::operation::UpdateSelections {
                    replica_id: lamport_timestamp.replica_id.as_u16() as u32,
                    lamport_timestamp: lamport_timestamp.value,
                    selections: serialize_selections(selections),
                    line_mode: *line_mode,
                    cursor_shape: serialize_cursor_shape(cursor_shape) as i32,
                },
            ),

            crate::Operation::UpdateDiagnostics {
                lamport_timestamp,
                server_id,
                diagnostics,
            } => project_models::operation::Variant::UpdateDiagnostics(
                project_models::UpdateDiagnostics {
                    replica_id: lamport_timestamp.replica_id.as_u16() as u32,
                    lamport_timestamp: lamport_timestamp.value,
                    server_id: server_id.0 as u64,
                    diagnostics: serialize_diagnostics(diagnostics.iter()),
                },
            ),

            crate::Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp,
                server_id,
            } => project_models::operation::Variant::UpdateCompletionTriggers(
                project_models::operation::UpdateCompletionTriggers {
                    replica_id: lamport_timestamp.replica_id.as_u16() as u32,
                    lamport_timestamp: lamport_timestamp.value,
                    triggers: triggers.clone(),
                    language_server_id: server_id.to_proto(),
                },
            ),

            crate::Operation::UpdateLineEnding {
                line_ending,
                lamport_timestamp,
            } => project_models::operation::Variant::UpdateLineEnding(
                project_models::operation::UpdateLineEnding {
                    replica_id: lamport_timestamp.replica_id.as_u16() as u32,
                    lamport_timestamp: lamport_timestamp.value,
                    line_ending: serialize_line_ending(*line_ending) as i32,
                },
            ),
        }),
    }
}

/// Serializes an [`EditOperation`] to be sent over RPC.
pub fn serialize_edit_operation(operation: &EditOperation) -> project_models::operation::Edit {
    project_models::operation::Edit {
        replica_id: operation.timestamp.replica_id.as_u16() as u32,
        lamport_timestamp: operation.timestamp.value,
        version: serialize_version(&operation.version),
        ranges: operation.ranges.iter().map(serialize_range).collect(),
        new_text: operation
            .new_text
            .iter()
            .map(|text| text.to_string())
            .collect(),
    }
}

/// Serializes an entry in the undo map to be sent over RPC.
pub fn serialize_undo_map_entry(
    (edit_id, counts): (&clock::Lamport, &[(clock::Lamport, u32)]),
) -> project_models::UndoMapEntry {
    project_models::UndoMapEntry {
        replica_id: edit_id.replica_id.as_u16() as u32,
        local_timestamp: edit_id.value,
        counts: counts
            .iter()
            .map(|(undo_id, count)| project_models::UndoCount {
                replica_id: undo_id.replica_id.as_u16() as u32,
                lamport_timestamp: undo_id.value,
                count: *count,
            })
            .collect(),
    }
}

/// Splits the given list of operations into chunks.
pub fn split_operations(
    mut operations: Vec<project_models::Operation>,
) -> impl Iterator<Item = Vec<project_models::Operation>> {
    #[cfg(any(test, feature = "test-support"))]
    const CHUNK_SIZE: usize = 5;

    #[cfg(not(any(test, feature = "test-support")))]
    const CHUNK_SIZE: usize = 100;

    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }

        let operations = operations
            .drain(..std::cmp::min(CHUNK_SIZE, operations.len()))
            .collect::<Vec<_>>();
        if operations.is_empty() {
            done = true;
        }
        Some(operations)
    })
}

/// Serializes selections to be sent over RPC.
pub fn serialize_selections(
    selections: &Arc<[Selection<Anchor>]>,
) -> Vec<project_models::Selection> {
    selections.iter().map(serialize_selection).collect()
}

/// Serializes a [`Selection`] to be sent over RPC.
pub fn serialize_selection(selection: &Selection<Anchor>) -> project_models::Selection {
    project_models::Selection {
        id: selection.id as u64,
        start: Some(project_models::EditorAnchor {
            anchor: Some(serialize_anchor(&selection.start)),
            excerpt_id: None,
        }),
        end: Some(project_models::EditorAnchor {
            anchor: Some(serialize_anchor(&selection.end)),
            excerpt_id: None,
        }),
        reversed: selection.reversed,
    }
}

/// Serializes a [`CursorShape`] to be sent over RPC.
pub fn serialize_cursor_shape(cursor_shape: &CursorShape) -> project_models::CursorShape {
    match cursor_shape {
        CursorShape::Bar => project_models::CursorShape::CursorBar,
        CursorShape::Block => project_models::CursorShape::CursorBlock,
        CursorShape::Underline => project_models::CursorShape::CursorUnderscore,
        CursorShape::Hollow => project_models::CursorShape::CursorHollow,
    }
}

/// Deserializes a [`CursorShape`] from the RPC representation.
pub fn deserialize_cursor_shape(cursor_shape: project_models::CursorShape) -> CursorShape {
    match cursor_shape {
        project_models::CursorShape::CursorBar => CursorShape::Bar,
        project_models::CursorShape::CursorBlock => CursorShape::Block,
        project_models::CursorShape::CursorUnderscore => CursorShape::Underline,
        project_models::CursorShape::CursorHollow => CursorShape::Hollow,
    }
}

/// Serializes a list of diagnostics to be sent over RPC.
pub fn serialize_diagnostics<'a>(
    diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<Anchor>>,
) -> Vec<project_models::Diagnostic> {
    diagnostics
        .into_iter()
        .map(|entry| {
            let lsp_markup = entry.diagnostic.message.lsp_markup();
            project_models::Diagnostic {
                source: entry.diagnostic.source.clone(),
                source_kind: match entry.diagnostic.source_kind {
                    DiagnosticSourceKind::Pulled => project_models::diagnostic::SourceKind::Pulled,
                    DiagnosticSourceKind::Pushed => project_models::diagnostic::SourceKind::Pushed,
                    DiagnosticSourceKind::Other => project_models::diagnostic::SourceKind::Other,
                } as i32,
                start: Some(serialize_anchor(&entry.range.start)),
                end: Some(serialize_anchor(&entry.range.end)),
                message: entry.diagnostic.message.to_string(),
                markdown: match lsp_markup {
                    Some(_) => None,
                    None => entry.diagnostic.message.markdown().map(ToOwned::to_owned),
                },
                markup_message_kind: lsp_markup.map(|(kind, _)| serialize_markup_kind(kind) as i32),
                untrimmed_markup_message: lsp_markup.and_then(|(_, value)| {
                    (value != entry.diagnostic.message.as_str()).then(|| value.to_string())
                }),
                severity: match entry.diagnostic.severity {
                    DiagnosticSeverity::ERROR => project_models::diagnostic::Severity::Error,
                    DiagnosticSeverity::WARNING => project_models::diagnostic::Severity::Warning,
                    DiagnosticSeverity::INFORMATION => {
                        project_models::diagnostic::Severity::Information
                    }
                    DiagnosticSeverity::HINT => project_models::diagnostic::Severity::Hint,
                    _ => project_models::diagnostic::Severity::None,
                } as i32,
                group_id: entry.diagnostic.group_id as u64,
                is_primary: entry.diagnostic.is_primary,
                underline: entry.diagnostic.underline,
                code: entry.diagnostic.code.as_ref().map(|s| s.to_string()),
                code_description: entry
                    .diagnostic
                    .code_description
                    .as_ref()
                    .map(|s| s.to_string()),
                is_disk_based: entry.diagnostic.is_disk_based,
                is_unnecessary: entry.diagnostic.is_unnecessary,
                data: entry.diagnostic.data.as_ref().map(|data| data.to_string()),
                registration_id: entry
                    .diagnostic
                    .registration_id
                    .as_ref()
                    .map(ToString::to_string),
            }
        })
        .collect()
}

/// Serializes an [`Anchor`] to be sent over RPC.
pub fn serialize_anchor(anchor: &Anchor) -> project_models::Anchor {
    let timestamp = anchor.timestamp();
    project_models::Anchor {
        replica_id: timestamp.replica_id.as_u16() as u32,
        timestamp: timestamp.value,
        offset: anchor.offset as u64,
        bias: match anchor.bias {
            Bias::Left => project_models::Bias::Left as i32,
            Bias::Right => project_models::Bias::Right as i32,
        },
        buffer_id: Some(anchor.buffer_id.into()),
    }
}

pub fn serialize_anchor_range(range: Range<Anchor>) -> project_models::AnchorRange {
    project_models::AnchorRange {
        start: Some(serialize_anchor(&range.start)),
        end: Some(serialize_anchor(&range.end)),
    }
}

/// Deserializes an [`Range<Anchor>`] from the RPC representation.
pub fn deserialize_anchor_range(range: project_models::AnchorRange) -> Result<Range<Anchor>> {
    Ok(
        deserialize_anchor(range.start.context("invalid anchor")?).context("invalid anchor")?
            ..deserialize_anchor(range.end.context("invalid anchor")?).context("invalid anchor")?,
    )
}

// This behavior is currently copied in the collab database, for snapshotting channel notes
/// Deserializes an [`crate::Operation`] from the RPC representation.
pub fn deserialize_operation(message: project_models::Operation) -> Result<crate::Operation> {
    Ok(
        match message.variant.context("missing operation variant")? {
            project_models::operation::Variant::Edit(edit) => {
                crate::Operation::Buffer(text::Operation::Edit(deserialize_edit_operation(edit)))
            }
            project_models::operation::Variant::Undo(undo) => {
                crate::Operation::Buffer(text::Operation::Undo(UndoOperation {
                    timestamp: clock::Lamport {
                        replica_id: ReplicaId::new(undo.replica_id as u16),
                        value: undo.lamport_timestamp,
                    },
                    version: deserialize_version(&undo.version),
                    counts: undo
                        .counts
                        .into_iter()
                        .map(|c| {
                            (
                                clock::Lamport {
                                    replica_id: ReplicaId::new(c.replica_id as u16),
                                    value: c.lamport_timestamp,
                                },
                                c.count,
                            )
                        })
                        .collect(),
                }))
            }
            project_models::operation::Variant::UpdateSelections(message) => {
                let selections = message
                    .selections
                    .into_iter()
                    .filter_map(|selection| {
                        Some(Selection {
                            id: selection.id as usize,
                            start: deserialize_anchor(selection.start?.anchor?)?,
                            end: deserialize_anchor(selection.end?.anchor?)?,
                            reversed: selection.reversed,
                            goal: SelectionGoal::None,
                        })
                    })
                    .collect::<Vec<_>>();

                crate::Operation::UpdateSelections {
                    lamport_timestamp: clock::Lamport {
                        replica_id: ReplicaId::new(message.replica_id as u16),
                        value: message.lamport_timestamp,
                    },
                    selections: Arc::from(selections),
                    line_mode: message.line_mode,
                    cursor_shape: deserialize_cursor_shape(
                        project_models::CursorShape::try_from(message.cursor_shape)
                            .ok()
                            .context("Missing cursor shape")?,
                    ),
                }
            }
            project_models::operation::Variant::UpdateDiagnostics(message) => {
                crate::Operation::UpdateDiagnostics {
                    lamport_timestamp: clock::Lamport {
                        replica_id: ReplicaId::new(message.replica_id as u16),
                        value: message.lamport_timestamp,
                    },
                    server_id: LanguageServerId(message.server_id as usize),
                    diagnostics: deserialize_diagnostics(message.diagnostics),
                }
            }
            project_models::operation::Variant::UpdateCompletionTriggers(message) => {
                crate::Operation::UpdateCompletionTriggers {
                    triggers: message.triggers,
                    lamport_timestamp: clock::Lamport {
                        replica_id: ReplicaId::new(message.replica_id as u16),
                        value: message.lamport_timestamp,
                    },
                    server_id: LanguageServerId::from_proto(message.language_server_id),
                }
            }
            project_models::operation::Variant::UpdateLineEnding(message) => {
                crate::Operation::UpdateLineEnding {
                    lamport_timestamp: clock::Lamport {
                        replica_id: ReplicaId::new(message.replica_id as u16),
                        value: message.lamport_timestamp,
                    },
                    line_ending: deserialize_line_ending(
                        project_models::LineEnding::try_from(message.line_ending)
                            .ok()
                            .context("missing line_ending")?,
                    ),
                }
            }
        },
    )
}

/// Deserializes an [`EditOperation`] from the RPC representation.
pub fn deserialize_edit_operation(edit: project_models::operation::Edit) -> EditOperation {
    EditOperation {
        timestamp: clock::Lamport {
            replica_id: ReplicaId::new(edit.replica_id as u16),
            value: edit.lamport_timestamp,
        },
        version: deserialize_version(&edit.version),
        ranges: edit.ranges.into_iter().map(deserialize_range).collect(),
        new_text: edit.new_text.into_iter().map(Arc::from).collect(),
    }
}

/// Deserializes an entry in the undo map from the RPC representation.
pub fn deserialize_undo_map_entry(
    entry: project_models::UndoMapEntry,
) -> (clock::Lamport, Vec<(clock::Lamport, u32)>) {
    (
        clock::Lamport {
            replica_id: ReplicaId::new(entry.replica_id as u16),
            value: entry.local_timestamp,
        },
        entry
            .counts
            .into_iter()
            .map(|undo_count| {
                (
                    clock::Lamport {
                        replica_id: ReplicaId::new(undo_count.replica_id as u16),
                        value: undo_count.lamport_timestamp,
                    },
                    undo_count.count,
                )
            })
            .collect(),
    )
}

/// Deserializes selections from the RPC representation.
pub fn deserialize_selections(
    selections: Vec<project_models::Selection>,
) -> Arc<[Selection<Anchor>]> {
    selections
        .into_iter()
        .filter_map(deserialize_selection)
        .collect()
}

/// Deserializes a [`Selection`] from the RPC representation.
pub fn deserialize_selection(selection: project_models::Selection) -> Option<Selection<Anchor>> {
    Some(Selection {
        id: selection.id as usize,
        start: deserialize_anchor(selection.start?.anchor?)?,
        end: deserialize_anchor(selection.end?.anchor?)?,
        reversed: selection.reversed,
        goal: SelectionGoal::None,
    })
}

pub fn serialize_markup_kind(kind: &lsp::MarkupKind) -> project_models::MarkupKind {
    match kind {
        lsp::MarkupKind::PlainText => project_models::MarkupKind::PlainText,
        lsp::MarkupKind::Markdown => project_models::MarkupKind::Markdown,
    }
}

pub fn deserialize_markup_kind(kind: i32) -> Option<lsp::MarkupKind> {
    match project_models::MarkupKind::try_from(kind).ok()? {
        project_models::MarkupKind::PlainText => Some(lsp::MarkupKind::PlainText),
        project_models::MarkupKind::Markdown => Some(lsp::MarkupKind::Markdown),
    }
}

/// Deserializes a list of diagnostics from the RPC representation.
pub fn deserialize_diagnostics(
    diagnostics: Vec<project_models::Diagnostic>,
) -> Arc<[DiagnosticEntry<Anchor>]> {
    diagnostics
        .into_iter()
        .filter_map(|diagnostic| {
            let data = if let Some(data) = diagnostic.data {
                Some(Value::from_str(&data).ok()?)
            } else {
                None
            };
            let message = match diagnostic
                .markup_message_kind
                .and_then(deserialize_markup_kind)
            {
                Some(kind) => DiagnosticMessage::from_lsp_markup(&lsp::MarkupContent {
                    kind,
                    value: diagnostic
                        .untrimmed_markup_message
                        .unwrap_or(diagnostic.message),
                }),
                None => DiagnosticMessage::plain_with_adapter_markdown(
                    diagnostic.message,
                    diagnostic.markdown.map(SharedString::from),
                ),
            };
            Some(DiagnosticEntry::new(
                deserialize_anchor(diagnostic.start?)?..deserialize_anchor(diagnostic.end?)?,
                Diagnostic {
                    source: diagnostic.source,
                    severity: match project_models::diagnostic::Severity::try_from(
                        diagnostic.severity,
                    )
                    .ok()?
                    {
                        project_models::diagnostic::Severity::Error => DiagnosticSeverity::ERROR,
                        project_models::diagnostic::Severity::Warning => {
                            DiagnosticSeverity::WARNING
                        }
                        project_models::diagnostic::Severity::Information => {
                            DiagnosticSeverity::INFORMATION
                        }
                        project_models::diagnostic::Severity::Hint => DiagnosticSeverity::HINT,
                        project_models::diagnostic::Severity::None => return None,
                    },
                    message,
                    group_id: diagnostic.group_id as usize,
                    code: diagnostic.code.map(lsp::NumberOrString::from_string),
                    code_description: diagnostic
                        .code_description
                        .and_then(|s| lsp::Uri::from_str(&s).ok()),
                    is_primary: diagnostic.is_primary,
                    is_disk_based: diagnostic.is_disk_based,
                    is_unnecessary: diagnostic.is_unnecessary,
                    underline: diagnostic.underline,
                    registration_id: diagnostic.registration_id.map(SharedString::from),
                    source_kind: match project_models::diagnostic::SourceKind::try_from(
                        diagnostic.source_kind,
                    )
                    .ok()?
                    {
                        project_models::diagnostic::SourceKind::Pulled => {
                            DiagnosticSourceKind::Pulled
                        }
                        project_models::diagnostic::SourceKind::Pushed => {
                            DiagnosticSourceKind::Pushed
                        }
                        project_models::diagnostic::SourceKind::Other => {
                            DiagnosticSourceKind::Other
                        }
                    },
                    data,
                },
            ))
        })
        .collect()
}

/// Deserializes an [`Anchor`] from the RPC representation.
pub fn deserialize_anchor(anchor: project_models::Anchor) -> Option<Anchor> {
    let buffer_id = if let Some(id) = anchor.buffer_id {
        Some(BufferId::new(id).ok()?)
    } else {
        None
    };
    let timestamp = clock::Lamport {
        replica_id: ReplicaId::new(anchor.replica_id as u16),
        value: anchor.timestamp,
    };
    let bias = match project_models::Bias::try_from(anchor.bias).ok()? {
        project_models::Bias::Left => Bias::Left,
        project_models::Bias::Right => Bias::Right,
    };
    Some(Anchor::new(
        timestamp,
        anchor.offset as u32,
        bias,
        buffer_id?,
    ))
}

/// Returns a `[clock::Lamport`] timestamp for the given [`project_models::Operation`].
pub fn lamport_timestamp_for_operation(
    operation: &project_models::Operation,
) -> Option<clock::Lamport> {
    let replica_id;
    let value;
    match operation.variant.as_ref()? {
        project_models::operation::Variant::Edit(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        project_models::operation::Variant::Undo(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        project_models::operation::Variant::UpdateDiagnostics(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        project_models::operation::Variant::UpdateSelections(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        project_models::operation::Variant::UpdateCompletionTriggers(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
        project_models::operation::Variant::UpdateLineEnding(op) => {
            replica_id = op.replica_id;
            value = op.lamport_timestamp;
        }
    }

    Some(clock::Lamport {
        replica_id: ReplicaId::new(replica_id as u16),
        value,
    })
}

/// Serializes a [`Transaction`] to be sent over RPC.
pub fn serialize_transaction(transaction: &Transaction) -> project_models::Transaction {
    project_models::Transaction {
        id: Some(serialize_timestamp(transaction.id)),
        edit_ids: transaction
            .edit_ids
            .iter()
            .copied()
            .map(serialize_timestamp)
            .collect(),
        start: serialize_version(&transaction.start),
    }
}

/// Deserializes a [`Transaction`] from the RPC representation.
pub fn deserialize_transaction(transaction: project_models::Transaction) -> Result<Transaction> {
    Ok(Transaction {
        id: deserialize_timestamp(transaction.id.context("missing transaction id")?),
        edit_ids: transaction
            .edit_ids
            .into_iter()
            .map(deserialize_timestamp)
            .collect(),
        start: deserialize_version(&transaction.start),
    })
}

/// Serializes a [`clock::Lamport`] timestamp to be sent over RPC.
pub fn serialize_timestamp(timestamp: clock::Lamport) -> project_models::LamportTimestamp {
    project_models::LamportTimestamp {
        replica_id: timestamp.replica_id.as_u16() as u32,
        value: timestamp.value,
    }
}

/// Deserializes a [`clock::Lamport`] timestamp from the RPC representation.
pub fn deserialize_timestamp(timestamp: project_models::LamportTimestamp) -> clock::Lamport {
    clock::Lamport {
        replica_id: ReplicaId::new(timestamp.replica_id as u16),
        value: timestamp.value,
    }
}

/// Serializes a range of [`FullOffset`]s to be sent over RPC.
pub fn serialize_range(range: &Range<FullOffset>) -> project_models::Range {
    project_models::Range {
        start: range.start.0 as u64,
        end: range.end.0 as u64,
    }
}

/// Deserializes a range of [`FullOffset`]s from the RPC representation.
pub fn deserialize_range(range: project_models::Range) -> Range<FullOffset> {
    FullOffset(range.start as usize)..FullOffset(range.end as usize)
}

/// Deserializes a clock version from the RPC representation.
pub fn deserialize_version(message: &[project_models::VectorClockEntry]) -> clock::Global {
    let mut version = clock::Global::new();
    for entry in message {
        version.observe(clock::Lamport {
            replica_id: ReplicaId::new(entry.replica_id as u16),
            value: entry.timestamp,
        });
    }
    version
}

/// Serializes a clock version to be sent over RPC.
pub fn serialize_version(version: &clock::Global) -> Vec<project_models::VectorClockEntry> {
    version
        .iter()
        .map(|entry| project_models::VectorClockEntry {
            replica_id: entry.replica_id.as_u16() as u32,
            timestamp: entry.value,
        })
        .collect()
}

pub fn serialize_lsp_edit(edit: lsp::TextEdit) -> project_models::TextEdit {
    let start = point_from_lsp(edit.range.start).0;
    let end = point_from_lsp(edit.range.end).0;
    project_models::TextEdit {
        new_text: edit.new_text,
        lsp_range_start: Some(project_models::PointUtf16 {
            row: start.row,
            column: start.column,
        }),
        lsp_range_end: Some(project_models::PointUtf16 {
            row: end.row,
            column: end.column,
        }),
    }
}

pub fn deserialize_lsp_edit(edit: project_models::TextEdit) -> Option<lsp::TextEdit> {
    let start = edit.lsp_range_start?;
    let start = PointUtf16::new(start.row, start.column);
    let end = edit.lsp_range_end?;
    let end = PointUtf16::new(end.row, end.column);
    Some(lsp::TextEdit {
        range: lsp::Range {
            start: point_to_lsp(start),
            end: point_to_lsp(end),
        },
        new_text: edit.new_text,
    })
}
