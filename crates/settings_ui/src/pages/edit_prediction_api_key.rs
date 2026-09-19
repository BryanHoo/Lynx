use edit_prediction::ApiKeyState;
use gpui::{App, Entity, TaskExt as _, prelude::*};
use ui::{ConfiguredApiCard, prelude::*};
use util::ResultExt as _;

use crate::{
    SettingsWindow,
    components::{SettingsInputField, SettingsSectionHeader},
};

pub(super) enum ApiKeyDocs {
    Custom { message: SharedString },
}

pub(super) fn render_api_key_provider(
    icon: IconName,
    title: &'static str,
    docs: ApiKeyDocs,
    api_key_state: Entity<ApiKeyState>,
    current_url: fn(&mut App) -> SharedString,
    additional_fields: Option<AnyElement>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let weak_page = cx.weak_entity();
    let credentials_provider = zed_credentials_provider::global(cx);
    _ = window.use_keyed_state(current_url(cx), cx, |_, cx| {
        let task = api_key_state.update(cx, |key_state, cx| {
            key_state.load_if_needed(
                current_url(cx),
                |state| state,
                credentials_provider.clone(),
                cx,
            )
        });
        cx.spawn(async move |_, cx| {
            task.await.log_err();
            weak_page.update(cx, |_, cx| cx.notify()).log_err();
        })
    });

    let (has_key, env_var_name, is_from_env_var) = api_key_state.read_with(cx, |state, _| {
        (
            state.has_key(),
            Some(state.env_var_name().clone()),
            state.is_from_env_var(),
        )
    });
    let write_key = move |api_key: Option<String>, cx: &mut App| {
        let credentials_provider = zed_credentials_provider::global(cx);
        api_key_state
            .update(cx, |key_state, cx| {
                key_state.store(
                    current_url(cx),
                    api_key,
                    |key_state| key_state,
                    credentials_provider,
                    cx,
                )
            })
            .detach_and_log_err(cx);
    };

    let container = v_flex().id(title).min_w_0().pt_8().gap_1p5();
    let header = SettingsSectionHeader::new(title)
        .icon(icon)
        .no_padding(true);
    let description = match docs {
        ApiKeyDocs::Custom { message } => div().min_w_0().w_full().child(
            Label::new(message)
                .size(LabelSize::Small)
                .color(Color::Muted),
        ),
    };

    let configured_label = if is_from_env_var {
        "API Key Set in Environment Variable"
    } else {
        "API Key Configured"
    };
    let container = if has_key {
        container.child(header).child(
            ConfiguredApiCard::new(format!("{title}-reset-key"), configured_label)
                .button_label("Reset Key")
                .button_tab_index(0)
                .disabled(is_from_env_var)
                .when_some(env_var_name, |this, env_var_name| {
                    this.when(is_from_env_var, |this| {
                        this.tooltip_label(format!(
                            "To reset your API key, unset the {} environment variable.",
                            env_var_name
                        ))
                    })
                })
                .on_click(move |_, _, cx| write_key(None, cx)),
        )
    } else {
        container.child(header).child(
            h_flex()
                .pt_2p5()
                .w_full()
                .min_w_0()
                .justify_between()
                .child(
                    v_flex()
                        .w_full()
                        .min_w_0()
                        .max_w_1_2()
                        .gap_0p5()
                        .child(Label::new("API Key"))
                        .child(description)
                        .when_some(env_var_name, |this, env_var_name| {
                            this.child(
                                Label::new(format!(
                                    "Or set the {} env var and restart Lynx.",
                                    env_var_name.as_ref()
                                ))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            )
                        }),
                )
                .child(
                    SettingsInputField::new(format!("{}-api-key-input", title))
                        .tab_index(0)
                        .with_placeholder("xxxxxxxxxxxxxxxxxxxx")
                        .aria_label(format!("{} API Key", title))
                        .on_confirm(move |api_key, _window, cx| {
                            write_key(api_key.filter(|key| !key.is_empty()), cx);
                        }),
                ),
        )
    };

    container.when_some(additional_fields, |this, additional_fields| {
        this.child(
            div()
                .map(|this| if has_key { this.mt_1() } else { this.mt_4() })
                .px_neg_8()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(additional_fields),
        )
    })
}
