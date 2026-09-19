// Generated from the reviewed zh-CN catalog; see THIRD_PARTY_NOTICES.md.
pub(super) fn translate(message: &str) -> Option<&'static str> {
    Some(match message {
        "{provider_name} API Key" => "{provider_name} API 密钥",
        "{provider_name} dashboard" => "{provider_name} 仪表盘",
        "{provider_name} dashboard." => "{provider_name} 仪表盘。",
        "{title} dashboard" => "{title} 仪表盘",
        "{} API Key" => "{} API 密钥",
        "{} Tool" => "{} 工具",
        "{} invalid" => "{} 个无效模式",
        "{} rules" => "{} 条规则",
        "{} tools" => "{} 个工具",
        _ => return None,
    })
}
