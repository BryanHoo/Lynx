use heck::ToTitleCase as _;
use strum::VariantNames;

// 这些值是产品名、协议名或用户熟悉的技术术语，界面中应保留原文。
const INTENTIONALLY_UNTRANSLATED: &[&str] = &[
    "Auto",
    "Alt",
    "Atom",
    "Bash",
    "CRLF",
    "Code Gemma",
    "Code Llama",
    "Codestral",
    "Default",
    "Deepseek Coder",
    "Emacs",
    "Fish",
    "Glm",
    "Helix",
    "Jet Brains",
    "LF",
    "MacOS",
    "Nushell",
    "PowerShell",
    "Qwen",
    "Star Coder",
    "Sublime Text",
    "Text Mate",
    "Unicode",
    "Vim",
    "Vs Code",
    "VSCode",
    "Zed",
    "Zeta",
    "Zeta2",
    "Zeta2 1",
    "Zsh",
];

fn record_missing<T: VariantNames>(missing: &mut Vec<String>) {
    for label in T::VARIANTS {
        let display_label = label.to_title_case();
        if !i18n::has_simplified_chinese_translation(&display_label)
            && !INTENTIONALLY_UNTRANSLATED.contains(&display_label.as_str())
        {
            missing.push(display_label);
        }
    }
}

#[test]
fn every_registered_settings_dropdown_option_has_a_translation() {
    let mut missing = Vec::new();

    macro_rules! check {
        ($($type:ty),+ $(,)?) => {
            $(record_missing::<$type>(&mut missing);)+
        };
    }

    check!(
        settings::CursorShape,
        settings::RestoreOnStartupBehavior,
        settings::OnNewWindow,
        settings::BottomDockLayout,
        settings::OnLastWindowClosed,
        settings::CliDefaultOpenBehavior,
        settings::DefaultOpenBehavior,
        settings::CloseWindowWhenNoItems,
        settings::TextRenderingMode,
        settings::BaseKeymapContent,
        settings::MultiCursorModifier,
        settings::HideMouseMode,
        settings::ReduceMotionMode,
        settings::CurrentLineHighlight,
        settings::ShowWhitespaceSetting,
        settings::SoftWrap,
        settings::AutoIndentMode,
        settings::ScrollBeyondLastLine,
        settings::SnippetSortOrder,
        settings::ClosePosition,
        settings::DockSide,
        settings::TerminalDockPosition,
        settings::DockPosition,
        settings::SidebarDockPosition,
        settings::GitGutterSetting,
        settings::GitHunkStyleSetting,
        settings::GitDiffBaseSetting,
        settings::GitPathStyle,
        settings::InlineBlameLocation,
        settings::DiagnosticSeverityContent,
        settings::SeedQuerySetting,
        settings::DoubleClickInMultibuffer,
        settings::GoToDefinitionFallback,
        settings::GoToDefinitionScrollStrategy,
        settings::OpenResultsIn,
        settings::ActivateOnClose,
        settings::ShowDiagnostics,
        settings::ShowCloseButton,
        settings::FolderIndicator,
        settings::ProjectPanelEntrySpacing,
        settings::ProjectPanelSortMode,
        settings::ProjectPanelSortOrder,
        settings::RewrapBehavior,
        settings::FormatOnSave,
        settings::LineEndingSetting,
        settings::IndentGuideColoring,
        settings::IndentGuideBackgroundColoring,
        settings::WordsCompletionMode,
        settings::LspInsertMode,
        settings::CompletionDetailAlignment,
        settings::CompletionMenuItemKind,
        settings::DiffViewStyle,
        settings::AlternateScroll,
        settings::TerminalBlink,
        settings::CursorShapeContent,
        settings::EditPredictionPromptFormatContent,
        settings::ShowScrollbar,
        settings::ScrollbarDiagnostics,
        settings::ShowMinimap,
        settings::DisplayIn,
        settings::MinimapThumb,
        settings::MinimapThumbBorder,
        settings::ModeContent,
        settings::UseSystemClipboard,
        settings::VimInsertModeCursorShape,
        settings::SteppingGranularity,
        settings::NotifyWhenAgentWaiting,
        settings::PlaySoundWhenAgentDone,
        settings::ThinkingBlockDisplay,
        settings::ImageFileSizeUnit,
        settings::StatusStyle,
        settings::GitPanelClickBehavior,
        settings::GitPanelSortBy,
        settings::GitPanelGroupBy,
        settings::EncodingDisplayOptions,
        settings::PaneSplitDirectionHorizontal,
        settings::PaneSplitDirectionVertical,
        settings::CodeLens,
        settings::DocumentColorsRenderMode,
        settings::ThemeSelectionDiscriminants,
        settings::ThemeAppearanceMode,
        settings::IconThemeSelectionDiscriminants,
        settings::BufferLineHeightDiscriminants,
        settings::GitGutterWidthDiscriminants,
        settings::ProjectPanelTitleTooltipDelayDiscriminants,
        settings::AutosaveSettingDiscriminants,
        settings::WorkingDirectoryDiscriminants,
        settings::IncludeIgnoredContent,
        settings::ShowIndentGuides,
        settings::ShellDiscriminants,
        settings::EditPredictionsMode,
        settings::RelativeLineNumbers,
        settings::WindowDecorations,
        settings::FullscreenMode,
        settings::WindowButtonLayoutContentDiscriminants,
        settings::ScanSymlinksSetting,
        settings::SemanticTokens,
        settings::DocumentFoldingRanges,
        settings::DocumentSymbols,
        settings::TerminalBell,
    );

    missing.sort();
    missing.dedup();
    assert!(
        missing.is_empty(),
        "缺少设置下拉选项翻译：\n{}",
        missing.join("\n")
    );
}
