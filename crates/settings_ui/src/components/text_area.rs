use std::rc::Rc;

use editor::Editor;
use gpui::{AccessibleAction, ElementId, Focusable, MouseButton, Role, accesskit::ActionData};
use language::language_settings::SoftWrap;
use ui::prelude::*;

use super::text_field_a11y_state;

type ConfirmHandler = Rc<dyn Fn(Option<String>, &mut Window, &mut App)>;

/// 用于编辑多行设置值的固定高度输入框。
#[derive(IntoElement)]
pub struct SettingsTextArea {
    id: ElementId,
    initial_text: Option<String>,
    placeholder: Option<&'static str>,
    confirm: Option<ConfirmHandler>,
    tab_index: Option<isize>,
    rows: usize,
    aria_label: Option<SharedString>,
    aria_description: Option<SharedString>,
}

impl SettingsTextArea {
    pub fn new(id: impl Into<ElementId>, rows: usize) -> Self {
        Self {
            id: id.into(),
            initial_text: None,
            placeholder: None,
            confirm: None,
            tab_index: None,
            rows: rows.max(1),
            aria_label: None,
            aria_description: None,
        }
    }

    pub fn with_initial_text(mut self, initial_text: String) -> Self {
        self.initial_text = Some(initial_text);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn on_confirm(
        mut self,
        confirm: impl Fn(Option<String>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.confirm = Some(Rc::new(confirm));
        self
    }

    pub(crate) fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = Some(tab_index);
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn aria_description(mut self, description: impl Into<SharedString>) -> Self {
        self.aria_description = Some(description.into());
        self
    }
}

impl RenderOnce for SettingsTextArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let synced_initial_text = window.use_keyed_state(
            (self.id.clone(), "first-render-initial-text"),
            cx,
            |_, _| self.initial_text.clone(),
        );

        let editor = window.use_keyed_state((self.id.clone(), "editor"), cx, {
            let initial_text = self.initial_text.clone();
            let placeholder = self.placeholder;
            let rows = self.rows;

            move |window, cx| {
                let mut editor = Editor::auto_height(rows, rows, window, cx);
                editor.set_show_gutter(false, cx);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);

                if let Some(text) = initial_text {
                    editor.set_text(text, window, cx);
                }
                if let Some(placeholder) = placeholder {
                    editor.set_placeholder_text(placeholder, window, cx);
                }

                editor
            }
        });

        // 将焦点订阅与编辑器一起保存在 keyed window state 中，确保设置页重绘时
        // 不会替换订阅或丢失内部 Editor 的焦点状态。
        let _blur_subscription =
            window.use_keyed_state((self.id.clone(), "blur-subscription"), cx, {
                let focus_handle = editor.focus_handle(cx);
                let weak_editor = editor.downgrade();
                let confirm = self.confirm.clone();
                move |window, cx| {
                    window.on_focus_out(&focus_handle, cx, move |_, window, cx| {
                        let Some(editor) = weak_editor.upgrade() else {
                            return;
                        };
                        let text = editor.read_with(cx, |editor, cx| editor.text(cx));
                        if let Some(confirm) = confirm.as_ref() {
                            confirm((!text.is_empty()).then_some(text), window, cx);
                        }
                    })
                }
            });

        let is_focused = editor.read(cx).is_focused(window);
        let editor_text = editor.read(cx).text(cx);

        // 设置文件重载时 keyed editor 仍会保留；同步外部值时必须避开正在输入的内容，
        // 否则一次设置刷新就会覆盖用户尚未失焦保存的文本。
        let synced_text = synced_initial_text.read(cx);
        if &self.initial_text != synced_text {
            let has_unsaved_edits = editor_text != synced_text.as_deref().unwrap_or_default();
            if !is_focused || !has_unsaved_edits {
                *synced_initial_text.as_mut(cx) = self.initial_text.clone();
                let weak_editor = editor.downgrade();
                let new_text = self.initial_text.clone().unwrap_or_default();
                window.defer(cx, move |window, cx| {
                    weak_editor
                        .update(cx, |editor, cx| editor.set_text(new_text, window, cx))
                        .ok();
                });
            }
        }

        let aria_label = self
            .aria_label
            .or_else(|| self.placeholder.map(SharedString::new_static));
        let (a11y_value, a11y_text_runs) =
            text_field_a11y_state(self.id.clone(), &editor, window, cx);
        let theme_colors = cx.theme().colors();
        let focus_handle = editor.focus_handle(cx);
        let focus_handle = if let Some(tab_index) = self.tab_index {
            focus_handle.tab_index(tab_index).tab_stop(true)
        } else {
            focus_handle
        };
        let pointer_focus_handle = focus_handle.clone();

        div()
            .id(self.id)
            .debug_selector(|| "settings-text-area".to_string())
            .role(Role::TextInput)
            .when_some(aria_label, |this, label| this.aria_label(label))
            .when_some(self.aria_description, |this, description| {
                this.aria_description(description)
            })
            .aria_value(a11y_value)
            .when_some(self.placeholder, |this, placeholder| {
                this.aria_placeholder(placeholder)
            })
            .a11y_synthetic_children(a11y_text_runs)
            .on_a11y_action(AccessibleAction::SetValue, {
                let weak_editor = editor.downgrade();
                let confirm = self.confirm;
                move |data, window, cx| {
                    let Some(ActionData::Value(text)) = data else {
                        return;
                    };
                    let Some(editor) = weak_editor.upgrade() else {
                        return;
                    };
                    let text = text.to_string();
                    editor.update(cx, |editor, cx| {
                        editor.set_text(text.clone(), window, cx);
                    });
                    if let Some(confirm) = confirm.as_ref() {
                        confirm((!text.is_empty()).then_some(text), window, cx);
                    }
                }
            })
            .w_full()
            .min_w_64()
            .px_2()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(theme_colors.border)
            .bg(theme_colors.editor_background)
            .overflow_hidden()
            .track_focus(&focus_handle)
            .focus(|style| style.border_color(theme_colors.border_focused))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                window.focus(&pointer_focus_handle, cx);
            })
            .child(editor)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gpui::{Context, FocusHandle, Modifiers, Render, TestAppContext};
    use settings::SettingsStore;

    use super::*;

    struct TextAreaTestView {
        saved_text: Rc<RefCell<Option<Option<String>>>>,
        blur_target: FocusHandle,
    }

    impl Render for TextAreaTestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let saved_text = self.saved_text.clone();

            div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    SettingsTextArea::new("test-text-area", 3).on_confirm(move |text, _, _| {
                        *saved_text.borrow_mut() = Some(text);
                    }),
                )
                .child(
                    div()
                        .h_8()
                        .track_focus(&self.blur_target)
                        .child("Blur target"),
                )
        }
    }

    #[gpui::test]
    fn text_area_should_accept_pointer_focus_and_save_on_blur(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let saved_text = Rc::new(RefCell::new(None));
        let (view, cx) = cx.add_window_view({
            let saved_text = saved_text.clone();
            move |_, cx| TextAreaTestView {
                saved_text,
                blur_target: cx.focus_handle(),
            }
        });
        cx.update(|window, _| window.activate_window());
        cx.run_until_parked();

        let text_area_bounds = cx
            .debug_bounds("settings-text-area")
            .expect("text area should be rendered");
        cx.update(|_, _| {});
        cx.simulate_mouse_move(text_area_bounds.center(), None, Modifiers::none());
        cx.simulate_click(text_area_bounds.center(), Modifiers::none());
        assert!(
            cx.update(|window, cx| window.focused(cx).is_some()),
            "click should focus the editor"
        );
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        cx.simulate_input("custom instructions");

        cx.update(|window, cx| {
            let blur_target = view.read(cx).blur_target.clone();
            window.focus(&blur_target, cx);
            let _ = window.draw(cx);
        });
        cx.run_until_parked();

        assert_eq!(
            saved_text.borrow().as_ref(),
            Some(&Some("custom instructions".to_string()))
        );
    }
}
