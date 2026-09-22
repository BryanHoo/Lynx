// Generated from the reviewed zh-CN catalog; see THIRD_PARTY_NOTICES.md.
pub(super) fn translate(message: &str) -> Option<&'static str> {
    Some(match message {
        "GitHub returned {} while fetching the skill" => "获取技能时 GitHub 返回 {}",
        "GitHub skill URLs must use https://" => "GitHub 技能 URL 必须使用 https://",
        "Global Substitution Default" => "全局替换默认值",
        "Global switch to toggle hints on and off." => "用于开启或关闭提示的全局开关",
        "Global switch to toggle inline values on and off when debugging." => {
            "调试时用于开关内联值显示的全局开关"
        }
        "Globs to match against file paths to determine if a file is private." => {
            "用于匹配文件路径以判断文件是否为私有的 glob 模式"
        }
        "Globs to match files that will be considered \"hidden\" and can be hidden from the project panel." => {
            "用于匹配被视为“隐藏”文件的 glob 模式，这些文件可在项目面板中隐藏"
        }
        "Go To Definition Fallback" => "转到定义回退",
        "Go To Definition Scroll Strategy" => "转到定义滚动策略",
        "Group By" => "分组依据",
        "Guides" => "参考线",
        "Gutter" => "装订线",
        "HTTP headers sent with each request to the server." => {
            "随每个请求发送到服务器的 HTTP 请求头"
        }
        "HTTP requests to URLs" => "对 URL 的 HTTP 请求",
        "Hard Tabs" => "硬制表符",
        "Headers" => "请求头",
        "Helix Mode" => "Helix 模式",
        "Hidden Files" => "隐藏文件",
        "Hide .gitignore" => "隐藏 .gitignore 匹配项",
        "Hide Hidden" => "隐藏隐藏项",
        "Hide Mouse" => "隐藏鼠标光标",
        "Hide Root" => "隐藏根目录",
        "Hide Symbols in Multi-Buffers" => "在多缓冲区中隐藏符号",
        "Hide the values of variables in private files." => "隐藏私有文件中变量的值",
        "Hide this skill from the model's catalog. It can still be invoked via slash command." => {
            "在模型目录中隐藏此技能，但仍可通过斜杠命令调用"
        }
        "Hiding Delay" => "隐藏延迟",
        "Highlight all occurrences of selected text." => "高亮显示所选文本的所有匹配项",
        "Highlight on Yank Duration" => "复制高亮持续时间",
        "Highlighting" => "高亮显示",
        "Horizontal Scroll" => "水平滚动",
        "Horizontal Scroll Margin" => "水平滚动边距",
        "Horizontal Scrollbar" => "水平滚动条",
        "Horizontal Split Direction" => "水平拆分方向",
        "Hover Popover" => "悬停弹出框",
        "How Git hunks are displayed visually in the editor." => "Git 区块在编辑器中的视觉显示方式",
        "How `zed <path>` opens directories when no flag is specified." => {
            "`zed <path>` 在未指定标志时打开目录的方式"
        }
        "How and when the scrollbar should be displayed." => "滚动条的显示方式和时机",
        "How entry statuses are displayed." => "条目状态的显示方式",
        "How line endings should be handled for new files and during format and save operations." => {
            "新文件及格式化和保存操作中换行符的处理方式"
        }
        "How long to wait for the server to respond before timing out." => {
            "超时前等待服务器响应的时长"
        }
        "How many characters has to be in the completions query to automatically show the words-based completions." => {
            "补全查询中需要多少个字符才能自动显示基于单词的补全"
        }
        "How many columns a tab should occupy." => "制表符应占用多少列",
        "How many lines of context to provide in multibuffer excerpts by default." => {
            "多缓冲区摘录中默认提供多少行上下文"
        }
        "How many lines to expand the multibuffer excerpts by default." => {
            "多缓冲区摘录默认展开多少行"
        }
        "How much to fade out unused code (0.0 - 0.9)." => "未使用代码的淡出程度（0.0 - 0.9）",
        "How projects open from the UI by default." => "项目从 UI 打开时的默认方式",
        "How thinking blocks should be displayed by default. 'Auto' fully expands during streaming, then auto-collapses when done. 'Preview' auto-expands with a height constraint during streaming. 'Always Expanded' shows full content. 'Always Collapsed' keeps them collapsed." => {
            "思考块的默认显示方式。“自动”在流式传输期间完全展开，完成后自动折叠。“预览”在流式传输期间自动展开，但高度受限。“始终展开”显示全部内容。“始终折叠”保持折叠状态。"
        }
        "How to display diffs in the editor." => "差异在编辑器中的显示方式",
        "How to display the LSP item kind (function, method, variable, etc.) of each entry in the completions menu." => {
            "补全菜单中各条目的 LSP 条目类型（函数、方法、变量等）的显示方式"
        }
        "How to group entries in the git panel." => "控制 Git 面板中条目的分组方式",
        "How to highlight the current line in the minimap." => "当前行在小地图中的高亮方式",
        "How to highlight the current line." => "当前行的高亮方式",
        "How to perform a buffer format." => "缓冲区格式化的执行方式",
        "How to render LSP color previews in the editor." => "LSP 颜色预览在编辑器中的渲染方式",
        "How to scroll the target into view when navigating to a definition or reference." => {
            "跳转到定义或引用时将目标滚动至视图的方式"
        }
        "How to soft-wrap long lines of text." => "长文本行的软换行方式",
        "How to sort entries in the git panel." => "控制 Git 面板中条目的排序方式",
        "Hunk Style" => "区块样式",
        "IP addresses and local domains aren't allowed; enter a domain like github.com." => {
            "不允许使用 IP 地址和本地域名；输入域名，例如 github.com."
        }
        "Icon Theme" => "图标主题",
        "Icon Theme Name" => "图标主题名称",
        "If any of these regexes match, a confirmation will be shown unless an Always Deny regex matches." => {
            "如果其中任一正则表达式匹配，将显示确认提示，除非匹配到“始终拒绝”规则"
        }
        "If any of these regexes match, the action will be approved—unless an Always Confirm or Always Deny matches." => {
            "如果其中任一正则表达式匹配，操作将被批准，除非匹配到“始终确认”或“始终拒绝”规则"
        }
        "If any of these regexes match, the tool action will be denied." => {
            "如果其中任一正则表达式匹配，工具操作将被拒绝"
        }
        "Image Viewer" => "图片查看器",
        "Import from URL" => "从 URL 导入",
        "Inactive Opacity" => "非活动窗格不透明度",
        "Include Ignored" => "包含已忽略",
        "Include Ignored in Search" => "搜索时包含已忽略",
        "Include Warnings" => "包含警告",
        "Include ignored files in search results by default." => "默认在搜索结果中包含已忽略文件",
        "Increment" => "递增",
        "Indent Guides" => "缩进参考线",
        "Indent Size" => "缩进大小",
        "Indentation" => "缩进",
        "Inlay Hints" => "内嵌提示",
        "Inline Code Actions" => "内联代码操作",
        "Inline Diagnostics" => "内联诊断",
        "Inline Git Blame" => "内联 Git 追溯",
        "Input Audio Device" => "音频输入设备",
        "Input Device" => "输入设备",
        "Insert Mode" => "插入模式",
        "Install from Extensions" => "从扩展安装",
        "Install from Registry" => "从注册中心安装",
        "Instrumentation" => "性能监测",
        "Invalid Patterns" => "无效模式",
        "Invalid URL in settings." => "设置中的 URL 无效",
        "Invalid URL: {error}" => "URL 无效：{error}",
        "Invalid regex: {err}. Pattern saved but will block this tool until fixed or removed." => {
            "无效正则表达式：{err}。模式已保存，但在修复或移除之前会阻止此工具。"
        }
        "JSX Tag Auto Close" => "JSX 标签自动关闭",
        "Keep Selection On Copy" => "复制后保留选区",
        "Key" => "键",
        "Key-value pairs to add to the terminal's environment." => "要添加到终端环境的键值对",
        "Keybindings" => "键绑定",
        "Keymap" => "键映射",
        "LLM Providers" => "LLM 提供商",
        "LSP Completions" => "LSP 补全",
        "LSP Document Colors" => "LSP 文档颜色",
        "LSP Document Symbols" => "LSP 文档符号",
        "LSP Folding Ranges" => "LSP 折叠范围",
        "LSP Highlights" => "LSP 高亮",
        "LSP Pull Diagnostics" => "LSP 拉取诊断",
        "LSP Results Location" => "LSP 结果位置",
        "Language Detection" => "语言检测",
        "Language Servers" => "语言服务器",
        "Languages" => "语言",
        "Languages & Tools" => "语言与工具",
        "Layout" => "布局",
        "Layout Settings" => "布局设置",
        "Layout mode for the bottom dock." => "底部停靠栏的布局模式",
        "Learn More" => "了解详情",
        "Learn more about sandboxing" => "了解沙盒的更多信息",
        "Left padding for centered layout." => "居中布局的左侧内边距",
        "Let sandboxed commands reach any domain over the network without prompting." => {
            "允许沙盒命令无需提示即可通过网络访问任何域名"
        }
        "Let sandboxed commands write anywhere except protected Git metadata without prompting." => {
            "允许沙盒命令无需提示即可写入任意位置，但受保护的 Git 元数据除外"
        }
        "Light Icon Theme" => "浅色图标主题",
        "Light Theme" => "浅色主题",
        "Limit Content Width" => "限制内容宽度",
        "Limit Markdown Preview Width" => "限制 Markdown 预览宽度",
        "Line Ending" => "换行符",
        "Line Endings Button" => "行尾按钮",
        "Line Height" => "行高",
        "Line Width" => "线宽",
        "Line height for editor text." => "编辑器文本的行高",
        "Line height for terminal text." => "终端文本的行高",
        "Linked Edits" => "联动编辑",
        "Loading agent skill instructions" => "加载智能体技能指令",
        "Loading models…" => "正在加载模型…",
        "Location" => "位置",
        "Log DAP Communications" => "记录 DAP 通信",
        "Log Out" => "退出登录",
        "MCP Server Timeout" => "MCP 服务器超时",
        "MCP Servers" => "MCP 服务器",
        "Manage Trust" => "管理信任",
        "Manage servers connected directly or via extensions." => {
            "管理直接连接或通过扩展连接的服务器"
        }
        "Markdown Preview Font" => "Markdown 预览字体",
        "Max Completion Tokens" => "最大生成 Token 数",
        "Max Content Width" => "最大内容宽度",
        "Max Output Tokens" => "最大输出 Token 数",
        "Max Scroll History Lines" => "最大滚动历史行数",
        "Max Severity" => "最大严重级别",
        "Max Tokens" => "最大 Token 数",
        "Max Width" => "最大宽度",
        "Max Width Columns" => "最大宽度列数",
        "Maximum Tabs" => "最大标签页数",
        "Maximum completion tokens for OpenAI-compatible requests." => {
            "OpenAI 兼容请求的最大生成 Token 数"
        }
        "Maximum content width in pixels. Content will be centered when the pane is wider than this value." => {
            "最大内容宽度（像素）。当窗格宽度超过此值时，内容将居中显示。"
        }
        "Maximum content width in pixels. Content will be centered when the panel is wider than this value." => {
            "内容最大宽度（像素）。当面板宽度超过此值时，内容将居中显示"
        }
        "Maximum directory depth to eagerly index outside of git repositories; contents of directories at this depth or deeper are indexed on demand. Repositories rooted shallower than this depth are always indexed fully. In projects that are not rooted at a git repository, repositories directly inside a root folder activate their git features immediately; deeper ones activate on first use. 0 means no limit and activates all git repositories immediately" => {
            "在 Git 仓库之外预先索引的最大目录深度；此深度或更深目录中的内容将按需索引。根目录深度小于此值的仓库始终会被完整索引。对于根目录不是 Git 仓库的项目，直接位于根文件夹内的仓库会立即激活其 Git 功能；更深层的仓库会在首次使用时激活。0 表示不限制，并立即激活所有 Git 仓库。"
        }
        "Maximum length of the commit message title before a warning is shown. Set to 0 to disable." => {
            "显示警告前提交消息标题的最大长度。设为 0 可禁用"
        }
        "Maximum number of columns to display in the minimap." => "小地图中显示的最大列数",
        "Maximum number of lines to keep in scrollback history (max: 100,000; 0 disables scrolling)." => {
            "滚动历史中保留的最大行数（最大值：100,000；0 表示禁用滚动）"
        }
        "Maximum open tabs in a pane. Will not close an unsaved tab." => {
            "窗格中的最大打开标签页数。不会关闭未保存的标签页"
        }
        "Menu Delay" => "菜单延迟",
        "Message Editor Min Lines" => "消息编辑器最少行数",
        "Middle Click Paste" => "中键粘贴",
        "Min Line Number Digits" => "最少行号位数",
        "Minimap" => "小地图",
        "Minimizes the settings UI window." => "最小化设置界面窗口",
        "Minimum Column" => "最小列号",
        "Minimum Contrast" => "最小对比度",
        "Minimum Contrast For Highlights" => "高亮最小对比度",
        "Minimum Split Diff Width" => "分栏差异视图最小宽度",
        "Minimum number of characters to reserve space for in the gutter." => {
            "在装订线中预留空间的最少字符数"
        }
        "Minimum number of lines to display in the agent message editor." => {
            "智能体消息编辑器显示的最少行数"
        }
        "Minimum time to wait before pulling diagnostics from the language server(s)." => {
            "从语言服务器拉取诊断信息前等待的最短时间"
        }
        "Miscellaneous" => "其他",
        "Modal Editing" => "模态编辑",
        "Mode" => "模式",
        "Model" => "模型",
        "Model Name" => "模型名称",
        "Model Name cannot be empty" => "模型名称不能为空",
        "Model Names must be unique" => "模型名称必须唯一",
        "Model used to generate Git commit messages. When unset, the default fast model is preferred, then the default model." => {
            "用于生成 Git 提交信息的模型。未设置时优先使用默认快速模型，其次使用默认模型。"
        }
        "Models" => "模型",
        "Modifier key for adding multiple cursors." => "添加多个光标的修饰键",
        "Mouse Wheel Zoom" => "滚轮缩放",
        "Move Path" => "移动路径",
        "Multi Cursor Modifier" => "多光标修饰键",
        "Multibuffer" => "多缓冲区",
        "Mute On Join" => "加入时静音",
        "Name" => "名称",
        "Network" => "网络",
        "No MCP servers added yet. Click \"Add Server\" to get started." => {
            "尚未添加 MCP 服务器。点击“添加服务器”开始。"
        }
        "No Results" => "无结果",
        "No active project found. Open a workspace to manage MCP servers." => {
            "未找到活动项目。打开工作区以管理 MCP 服务器。"
        }
        "No active project found. Open a workspace to manage external agents." => {
            "未找到活动项目。打开工作区以管理外部智能体。"
        }
        "No external agents added yet. Click \"Add Agent\" to get started." => {
            "尚未添加外部智能体。点击“添加智能体”开始。"
        }
        "No global skills installed." => "未安装全局技能",
        "No models found. Check your Ollama server URL." => "未找到模型。检查 Ollama 服务器 URL。",
        "No patterns configured" => "未配置模式",
        "No project skills found." => "未找到项目技能",
        "No regex matches, using the default action." => "没有正则表达式匹配，使用默认操作",
        "No settings match \"{}\"" => "没有设置匹配 \"{}\"",
        "No skills available for this context." => "此上下文中没有可用技能",
        "Not a valid domain. Use a domain like github.com or *.npmjs.org." => {
            "不是有效域名。使用 github.com 或 *.npmjs.org 这样的域名。"
        }
        "Note: custom tool permissions only apply to the Zed native agent and don’t extend to external agents connected through the Agent Client Protocol (ACP)." => {
            "注意：自定义工具权限仅适用于 Lynx 原生智能体，不会扩展到通过 Agent Client Protocol (ACP) 连接的外部智能体"
        }
        "Nothing configured" => "未配置任何项",
        "Notify When Agent Waiting" => "智能体等待时通知",
        "Number of lines to search for modelines (set to 0 to disable)." => {
            "搜索模式行的行数（设为 0 可禁用）"
        }
        "OAuth Client ID" => "OAuth 客户端 ID",
        "On Last Window Closed" => "关闭最后一个窗口时",
        "On New Window" => "打开新窗口时",
        "On: format the whole buffer.\nOff: do not format.\nModifications: format only lines with unstaged changes; skips formatting when a git diff or LSP range formatting is unavailable.\nModifications If Available: same, but falls back to formatting the whole buffer." => {
            "开启：格式化整个缓冲区。\n关闭：不格式化。\n修改内容：仅格式化包含未暂存更改的行；当 Git 差异不可用或 LSP 不支持范围格式化时，跳过格式化。\n修改内容（若可用）：同上，但会回退到格式化整个缓冲区。"
        }
        "Opacity of inactive panels (0.0 - 1.0)." => "非活动窗格的不透明度（0.0 - 1.0）",
        "Open" => "打开",
        "Open Keymap" => "打开键映射",
        "Open Links In Mouse Mode" => "在鼠标模式下打开链接",
        "Open Markdown Files in Preview" => "在 Markdown 预览中打开 Markdown 文件",
        "Opens an editor for the current file" => "为当前文件打开编辑器",
        "Opens {docs_url}" => "打开 {docs_url}",
        "Optimize Lynx's interface for assistive technology such as screen readers. When enabled, otherwise-collapsed controls stay expanded and keyboard-reachable." => {
            "针对屏幕阅读器等辅助技术优化 Lynx 界面。启用后，原本折叠的控件会保持展开，并可通过键盘访问。"
        }
        "Optimize Zed's interface for assistive technology such as screen readers. When enabled, otherwise-collapsed controls stay expanded and keyboard-reachable." => {
            "针对屏幕阅读器等辅助技术优化 Lynx 的界面。启用后，原本会折叠的控件将保持展开，并可通过键盘访问。"
        }
        "Option As Meta" => "Option 键作为 Meta 键",
        "Optional OAuth client ID" => "可选 OAuth 客户端 ID",
        "Optional OAuth client ID used to authenticate with the server." => {
            "用于向服务器进行身份验证的可选 OAuth 客户端 ID"
        }
        "Options" => "选项",
        "Or set the {env_var_name} env var and restart Zed for it to take effect." => {
            "或者设置 {env_var_name} 环境变量并重启 Lynx 以使其生效"
        }
        "Or set the {} env var and restart Zed." => "或设置 {} 环境变量并重启 Lynx。",
        "Outline Panel" => "大纲面板",
        "Outline Panel Button" => "大纲面板按钮",
        _ => return None,
    })
}
