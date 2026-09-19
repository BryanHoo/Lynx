mod zh_cn;

use gpui::{App, SharedString};
use settings::{RegisterSetting, Settings, SettingsContent, UiLocale};
use std::sync::LazyLock;

// 系统 locale 在进程生命周期内保持稳定，缓存后可避免每次设置变更都访问平台 API。
static SYSTEM_LOCALE: LazyLock<Option<String>> = LazyLock::new(sys_locale::get_locale);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    English,
    SimplifiedChinese,
}

impl Locale {
    pub fn resolve(preference: UiLocale, system_locale: Option<&str>) -> Self {
        match preference {
            UiLocale::English => Self::English,
            UiLocale::SimplifiedChinese => Self::SimplifiedChinese,
            UiLocale::System => system_locale
                .filter(|locale| is_simplified_chinese(locale))
                .map_or(Self::English, |_| Self::SimplifiedChinese),
        }
    }
}

#[derive(RegisterSetting)]
pub struct LocalizationSettings {
    pub preference: UiLocale,
    pub locale: Locale,
}

impl Settings for LocalizationSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let preference = content.ui_locale.unwrap_or_default();

        Self {
            preference,
            locale: Locale::resolve(preference, SYSTEM_LOCALE.as_deref()),
        }
    }
}

pub fn locale(cx: &App) -> Locale {
    LocalizationSettings::get_global(cx).locale
}

/// 翻译只查询编译期静态目录；缺少词条时直接返回英文原文，避免空白界面。
pub fn translate(locale: Locale, message: &'static str) -> &'static str {
    match locale {
        Locale::English => message,
        Locale::SimplifiedChinese => zh_cn::translate(message).unwrap_or(message),
    }
}

pub fn translate_in(cx: &App, message: &'static str) -> &'static str {
    translate(locale(cx), message)
}

/// 动态设置标题无法保留 `'static` 生命周期；命中词典时仍复用静态字符串。
pub fn translate_shared_in(cx: &App, message: &str) -> SharedString {
    match locale(cx) {
        Locale::English => message.to_string().into(),
        Locale::SimplifiedChinese => {
            if let Some(translation) = zh_cn::translate(message) {
                SharedString::new_static(translation)
            } else if let Some(provider) = message
                .strip_prefix("Add ")
                .and_then(|message| message.strip_suffix("-Compatible Provider"))
            {
                format!("添加 {provider} 兼容提供商").into()
            } else if let Some(tool) = message.strip_suffix(" Tool") {
                let tool = zh_cn::translate(tool).unwrap_or(tool);
                format!("{tool}工具").into()
            } else {
                message.to_string().into()
            }
        }
    }
}

pub fn locale_labels(locale: Locale) -> &'static [&'static str] {
    const ENGLISH: &[&str] = &["System", "English", "Simplified Chinese"];
    const SIMPLIFIED_CHINESE: &[&str] = &["跟随系统", "English", "简体中文"];

    match locale {
        Locale::English => ENGLISH,
        Locale::SimplifiedChinese => SIMPLIFIED_CHINESE,
    }
}

pub fn format_no_settings_match(locale: Locale, query: &str) -> String {
    match locale {
        Locale::English => format!("No settings match \"{query}\""),
        Locale::SimplifiedChinese => format!("没有与“{query}”匹配的设置"),
    }
}

fn is_simplified_chinese(locale: &str) -> bool {
    let normalized = locale.to_ascii_lowercase().replace('_', "-");
    normalized == "zh"
        || normalized.starts_with("zh-cn")
        || normalized.starts_with("zh-sg")
        || normalized.starts_with("zh-hans")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_should_honor_explicit_locale() {
        assert_eq!(
            Locale::resolve(UiLocale::English, Some("zh-CN")),
            Locale::English
        );
        assert_eq!(
            Locale::resolve(UiLocale::SimplifiedChinese, Some("en-US")),
            Locale::SimplifiedChinese
        );
    }

    #[test]
    fn resolve_should_detect_simplified_chinese_system_locale() {
        assert_eq!(
            Locale::resolve(UiLocale::System, Some("zh-Hans-CN")),
            Locale::SimplifiedChinese
        );
        assert_eq!(
            Locale::resolve(UiLocale::System, Some("en-US")),
            Locale::English
        );
    }

    #[test]
    fn translate_should_use_chinese_catalog_and_english_fallback() {
        assert_eq!(
            translate(Locale::SimplifiedChinese, "Display Language"),
            "显示语言"
        );
        assert_eq!(
            translate(Locale::SimplifiedChinese, "Unregistered message"),
            "Unregistered message"
        );
        assert_eq!(
            translate(Locale::SimplifiedChinese, "LLM Providers"),
            "LLM 提供商"
        );
        assert_eq!(
            translate(
                Locale::SimplifiedChinese,
                "View, add, configure, and remove Model Context Protocol servers.",
            ),
            "查看、添加、配置和移除 Model Context Protocol 服务器"
        );
    }

    #[test]
    fn chinese_catalog_should_support_runtime_setting_titles() {
        let message = String::from("Tool Permissions");
        assert_eq!(zh_cn::translate(&message), Some("工具权限"));
    }

    #[test]
    fn ui_locale_should_use_stable_setting_values() {
        assert_eq!(
            serde_json::to_string(&UiLocale::System).unwrap(),
            "\"system\""
        );
        assert_eq!(serde_json::to_string(&UiLocale::English).unwrap(), "\"en\"");
        assert_eq!(
            serde_json::to_string(&UiLocale::SimplifiedChinese).unwrap(),
            "\"zh-CN\""
        );
    }
}
