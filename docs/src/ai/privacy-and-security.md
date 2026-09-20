---
title: AI Privacy - Lynx
description: Understand where Lynx sends AI requests and how to control access to project data.
---

# AI Privacy

Lynx does not provide a hosted model service. AI requests go directly to the
provider, gateway, local server, or external agent that you configure.

## AI Request Paths {#ai-request-paths}

| Path                                                  | Who handles requests                     | What to review                                                                   |
| ----------------------------------------------------- | ---------------------------------------- | -------------------------------------------------------------------------------- |
| [Provider API keys](./use-api-access.md)              | The configured provider                  | The provider's data retention, training, and account terms                       |
| [Gateways](./use-a-gateway.md)                        | The gateway and its upstream providers   | Both the gateway and upstream provider policies                                  |
| [Local models](./use-a-local-model.md)                | Your local or self-hosted server         | The server configuration and any network routes you expose                       |
| [External Agents](./external-agents.md)               | The agent and its configured providers   | The agent's authentication, tools, instructions, and model provider policies     |
| [Terminal Threads](./terminal-threads.md)             | The CLI or TUI running in the terminal   | The program's authentication, storage, tools, and model provider policies        |
| [Edit Prediction](./edit-prediction.md)               | Your configured prediction endpoint      | Editing context can be sent while you type                                       |
| [Agent tools](./tools.md) and [MCP servers](./mcp.md) | Lynx and the external systems you enable | Tools can read files, edit code, run commands, fetch URLs, and call integrations |

API keys saved through Lynx are stored in the system keychain rather than
`settings.json`.

## Project Access {#project-access}

Agent tools can access files and commands allowed by the active
[Agent Profile](./agent-profiles.md) and
[Tool Permissions](./tool-permissions.md). Project instructions, skills, and MCP
configuration can change agent behavior, so review them before trusting a
worktree.

See [Worktree Trust](../worktree-trust.md), [Skills](./skills.md), and
[Instructions](./instructions.md) for these boundaries.

## Disable AI {#disable-ai}

Open the Settings Editor with {#action zed::OpenSettings}, search for
`Disable AI`, and enable it. This disables the Threads Sidebar, Agent Panel,
Edit Prediction, and Inline Assistant.
