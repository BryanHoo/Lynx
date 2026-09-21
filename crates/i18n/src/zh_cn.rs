#[path = "zh_cn_options.rs"]
mod zh_cn_options;
#[path = "zh_cn_settings_1.rs"]
mod zh_cn_settings_1;
#[path = "zh_cn_settings_2.rs"]
mod zh_cn_settings_2;
#[path = "zh_cn_settings_3.rs"]
mod zh_cn_settings_3;
#[path = "zh_cn_settings_4.rs"]
mod zh_cn_settings_4;
#[path = "zh_cn_settings_5.rs"]
mod zh_cn_settings_5;
#[path = "zh_cn_settings_6.rs"]
mod zh_cn_settings_6;
#[path = "zh_cn_settings_7.rs"]
mod zh_cn_settings_7;

pub(super) fn translate(message: &str) -> Option<&'static str> {
    translate_core(message)
        .or_else(|| zh_cn_options::translate(message))
        .or_else(|| zh_cn_settings_1::translate(message))
        .or_else(|| zh_cn_settings_2::translate(message))
        .or_else(|| zh_cn_settings_3::translate(message))
        .or_else(|| zh_cn_settings_4::translate(message))
        .or_else(|| zh_cn_settings_5::translate(message))
        .or_else(|| zh_cn_settings_6::translate(message))
        .or_else(|| zh_cn_settings_7::translate(message))
}

fn translate_core(message: &str) -> Option<&'static str> {
    Some(match message {
        "Settings" => "设置",
        "Agent" => "智能体",
        "Authentication Required." => "需要身份验证。",
        "Client Secret Required." => "需要客户端密钥。",
        "Server has an error." => "服务器发生错误。",
        "Server is active." => "服务器正在运行。",
        "Server is starting." => "服务器正在启动。",
        "Server is stopped." => "服务器已停止。",
        "Waiting for Authorization…" => "正在等待授权…",
        "Note: custom tool permissions only apply to the Lynx native agent and don’t extend to external agents connected through the Agent Client Protocol (ACP)." => {
            "注意：自定义工具权限仅适用于 Lynx 原生智能体，不会扩展到通过 Agent Client Protocol (ACP) 连接的外部智能体"
        }
        "No provider set" => "未设置提供商",
        "Paste a GitHub .md URL to fetch it and fill out the form. For private files, Lynx retries using GITHUB_TOKEN, if set." => {
            "粘贴 GitHub .md URL 以获取内容并填写表单。对于私有文件，如果已设置 GITHUB_TOKEN，Lynx 会使用它重试。"
        }
        "Wrap agent-run terminal commands in an OS-level sandbox. When off, commands run with Lynx's own permissions." => {
            "将智能体运行的终端命令置于操作系统级沙盒中。关闭后，命令将使用 Lynx 自身的权限运行。"
        }
        "Search settings…" => "搜索设置…",
        "General" => "常规",
        "General Settings" => "常规设置",
        "Display Language" => "显示语言",
        "Current Workspace" => "当前工作区",
        "Choose the language used by Lynx's interface." => "选择 Lynx 界面使用的语言。",
        "Accessible Mode" => "无障碍模式",
        "When Closing With No Tabs" => "无标签页时关闭",
        "On New Window" => "新建窗口时",
        "On Last Window Closed" => "关闭最后一个窗口时",
        "Use System Path Prompts" => "使用系统路径对话框",
        "Use System Prompts" => "使用系统确认对话框",
        "Redact Private Values" => "隐藏私密值",
        "Private Files" => "私密文件",
        "CLI Default Open Behavior" => "CLI 默认打开行为",
        "Reveal If Open" => "定位已打开文件",
        "Default Open Behavior" => "默认打开行为",
        "Trust All Projects By Default" => "默认信任所有项目",
        "Restore Unsaved Buffers" => "恢复未保存缓冲区",
        "Restore On Startup" => "启动时恢复",
        "Preview Channel" => "预览渠道",
        "Settings Profiles" => "设置配置文件",
        "Anthropic Data Retention" => "Anthropic 数据保留",
        "Appearance" => "外观",
        "Keymap" => "键位映射",
        "Editor" => "编辑器",
        "Languages & Tools" => "语言与工具",
        "Search & Files" => "搜索与文件",
        "Window & Layout" => "窗口与布局",
        "Panels" => "面板",
        "Terminal" => "终端",
        "Version Control" => "版本控制",
        "AI" => "AI",
        "Network" => "网络",
        "Developer" => "开发者",
        "Security" => "安全",
        "Workspace Restoration" => "工作区恢复",
        "Scoped Settings" => "作用域设置",
        "Privacy" => "隐私",
        "User" => "用户",
        "Project" => "项目",
        "Default" => "默认",
        "No Results" => "无结果",
        "Settings Content" => "设置内容",
        "Settings Navigation" => "设置导航",
        "Reset to Default" => "恢复默认值",
        "Copy Link" => "复制链接",
        _ => return None,
    })
}
