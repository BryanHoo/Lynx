use std::{sync::Arc, sync::LazyLock};

use anyhow::{Result, anyhow};
use editor::{Editor, MultiBufferOffset, SelectionEffects, scroll::Autoscroll};
use fs::Fs;
use gpui::{App, AsyncWindowContext, WeakEntity};
use language::{
    Language,
    language_settings::{
        AllLanguageSettings, EditPredictionProvider, EditPredictionsMode, all_language_settings,
    },
};
use regex::Regex;
use settings::{Settings as _, SettingsStore, update_settings_file};
use util::ResultExt as _;
use workspace::{Workspace, create_and_open_local_file};

pub(crate) async fn open_disabled_globs_setting_in_editor(
    workspace: WeakEntity<Workspace>,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let settings_item = workspace
        .update_in(cx, |_, window, cx| {
            create_and_open_local_file(paths::settings_file(), window, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?;
    let settings_editor = settings_item
        .downcast::<Editor>()
        .ok_or_else(|| anyhow!("settings file did not open in an editor"))?;

    settings_editor
        .downgrade()
        .update_in(cx, |item, window, cx| {
            let text = item.buffer().read(cx).snapshot(cx).text();
            let settings = cx.global::<SettingsStore>();
            let Some(edits) = settings
                .edits_for_update(&text, |file| {
                    file.project
                        .all_languages
                        .edit_predictions
                        .get_or_insert_with(Default::default)
                        .disabled_globs
                        .get_or_insert_with(Vec::new);
                })
                .log_err()
            else {
                return;
            };

            if !edits.is_empty() {
                item.edit(
                    edits.into_iter().map(|(range, text)| {
                        (
                            MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                            text,
                        )
                    }),
                    cx,
                );
            }

            let text = item.buffer().read(cx).snapshot(cx).text();
            static DISABLED_GLOBS_REGEX: LazyLock<Option<Regex>> = LazyLock::new(|| {
                Regex::new(r#""disabled_globs":\s*\[\s*(?P<content>(?:.|\n)*?)\s*\]"#).ok()
            });
            let range = DISABLED_GLOBS_REGEX
                .as_ref()
                .and_then(|regex| regex.captures(&text))
                .and_then(|captures| {
                    captures
                        .name("content")
                        .map(|inner_match| inner_match.start()..inner_match.end())
                });
            if let Some(range) = range {
                item.change_selections(
                    SelectionEffects::scroll(Autoscroll::newest()),
                    window,
                    cx,
                    |selections| {
                        selections.select_ranges(vec![
                            MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                        ]);
                    },
                );
            }
        })?;

    Ok(())
}

pub fn set_completion_provider(fs: Arc<dyn Fs>, cx: &mut App, provider: EditPredictionProvider) {
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
            .edit_predictions
            .get_or_insert_default()
            .provider = Some(provider);
    });
}

pub fn get_available_providers(cx: &mut App) -> Vec<EditPredictionProvider> {
    let settings = &all_language_settings(None, cx).edit_predictions;
    [
        settings
            .ollama
            .is_some()
            .then_some(EditPredictionProvider::Ollama),
        settings
            .open_ai_compatible_api
            .is_some()
            .then_some(EditPredictionProvider::OpenAiCompatibleApi),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub(crate) fn toggle_show_edit_predictions_for_language(
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let show_edit_predictions =
        all_language_settings(None, cx).show_edit_predictions(Some(&language), cx);
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
            .languages
            .0
            .entry(language.name().0.to_string())
            .or_default()
            .show_edit_predictions = Some(!show_edit_predictions);
    });
}

pub(crate) fn toggle_edit_prediction_mode(
    fs: Arc<dyn Fs>,
    mode: EditPredictionsMode,
    cx: &mut App,
) {
    if AllLanguageSettings::get_global(cx).edit_predictions_mode() == mode {
        return;
    }

    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
            .edit_predictions
            .get_or_insert_default()
            .mode = Some(mode);
    });
}
