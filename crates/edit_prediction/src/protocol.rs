use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, Serialize, Deserialize, strum::AsRefStr, strum::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum PredictEditsRequestTrigger {
    Testing,
    Diagnostics,
    DiagnosticNavigation,
    Cli,
    Explicit,
    BufferEdit,
    LSPCompletionAccepted,
    PredictionAccepted,
    PredictionPartiallyAccepted,
    EditorCreated,
    ProviderChanged,
    UserInfoChanged,
    VimModeChanged,
    SettingsChanged,
    #[default]
    Other,
}

#[derive(
    Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum EditPredictionRejectReason {
    Canceled,
    Empty,
    InterpolatedEmpty,
    InterpolateFailed,
    PatchApplyFailed,
    Replaced,
    CurrentPreferred,
    #[default]
    Discarded,
    Rejected,
}
