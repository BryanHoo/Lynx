use std::rc::Rc;

use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString};
use language_model::LanguageModelRegistry;
use settings::{LanguageModelProviderSetting, LanguageModelSelection};
use ui::{ButtonSize, ContextMenu, DropdownMenu, DropdownStyle, IconPosition, px};

type ChangeHandler = Rc<dyn Fn(Option<LanguageModelSelection>, &mut ui::Window, &mut App)>;

#[derive(Clone)]
struct ModelOption {
    label: SharedString,
    selection: Option<LanguageModelSelection>,
}

#[derive(IntoElement)]
pub struct SettingsLanguageModelDropdown {
    id: ElementId,
    options: Vec<ModelOption>,
    selected_index: usize,
    aria_label: SharedString,
    aria_description: SharedString,
    on_change: ChangeHandler,
}

impl SettingsLanguageModelDropdown {
    pub fn new(
        id: impl Into<ElementId>,
        current: Option<&LanguageModelSelection>,
        aria_label: impl Into<SharedString>,
        aria_description: impl Into<SharedString>,
        cx: &App,
        on_change: impl Fn(Option<LanguageModelSelection>, &mut ui::Window, &mut App) + 'static,
    ) -> Self {
        let options = Self::model_options(cx);
        let selected_index = Self::selected_index(&options, current);

        Self {
            id: id.into(),
            options,
            selected_index,
            aria_label: aria_label.into(),
            aria_description: aria_description.into(),
            on_change: Rc::new(on_change),
        }
    }

    fn selected_index(options: &[ModelOption], current: Option<&LanguageModelSelection>) -> usize {
        current
            .and_then(|current| {
                options.iter().position(|option| {
                    option.selection.as_ref().is_some_and(|selection| {
                        selection.provider == current.provider && selection.model == current.model
                    })
                })
            })
            .unwrap_or(0)
    }

    fn model_options(cx: &App) -> Vec<ModelOption> {
        let mut options = vec![ModelOption {
            label: i18n::translate_shared_in(cx, "Default Model (Prefer Fast Model)"),
            selection: None,
        }];

        let registry = LanguageModelRegistry::read_global(cx);
        for provider in registry
            .visible_providers()
            .into_iter()
            .filter(|provider| provider.is_authenticated(cx))
        {
            for model in provider.provided_models(cx) {
                // 使用供应商前缀区分同名模型，同时写入模型支持的默认参数。
                options.push(ModelOption {
                    label: format!("{} - {}", provider.name().0, model.name().0).into(),
                    selection: Some(LanguageModelSelection {
                        provider: LanguageModelProviderSetting(provider.id().0.to_string()),
                        model: model.id().0.to_string(),
                        enable_thinking: model.supports_thinking(),
                        effort: model
                            .default_effort_level()
                            .map(|effort| effort.value.to_string()),
                        speed: None,
                    }),
                });
            }
        }

        options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_option_should_be_selected_without_explicit_model() {
        let options = vec![
            ModelOption {
                label: "Default".into(),
                selection: None,
            },
            ModelOption {
                label: "Provider - Model".into(),
                selection: Some(LanguageModelSelection {
                    provider: LanguageModelProviderSetting("provider".to_string()),
                    model: "model".to_string(),
                    enable_thinking: false,
                    effort: None,
                    speed: None,
                }),
            },
        ];

        assert_eq!(
            SettingsLanguageModelDropdown::selected_index(&options, None),
            0
        );
    }
}

impl RenderOnce for SettingsLanguageModelDropdown {
    fn render(self, window: &mut ui::Window, cx: &mut App) -> impl IntoElement {
        let current_label = self.options[self.selected_index].label.clone();
        // 供应商配置变化时，即使当前选项未变化，也必须重建菜单内容。
        let options_key = self.options.iter().fold(String::new(), |mut key, option| {
            key.push_str(option.label.as_ref());
            key.push('\0');
            key
        });
        let menu_key = format!("{}:{options_key}", self.selected_index);
        let menu = window.use_keyed_state(menu_key, cx, |window, cx| {
            let options = self.options.clone();
            ContextMenu::new(window, cx, move |mut menu, _, _| {
                for (index, option) in options.iter().enumerate() {
                    let on_change = self.on_change.clone();
                    let selection = option.selection.clone();
                    menu = menu.toggleable_entry(
                        option.label.clone(),
                        index == self.selected_index,
                        IconPosition::End,
                        None,
                        move |window, cx| on_change(selection.clone(), window, cx),
                    );
                }
                menu
            })
        });

        DropdownMenu::new(self.id, current_label, menu)
            .aria_label(self.aria_label)
            .aria_description(self.aria_description)
            .tab_index(0)
            .trigger_size(ButtonSize::Medium)
            .style(DropdownStyle::Outlined)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            })
    }
}
