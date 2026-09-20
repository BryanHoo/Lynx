---
title: Use an Existing AI Subscription - Lynx
description: Use provider subscriptions through external agents and terminal tools that support them.
---

# Use an Existing AI Subscription

Lynx does not convert consumer AI subscriptions into API access. A subscription
works only when an External Agent or command-line tool supports that provider's
login flow.

| Subscription       | Lynx path                                     | Authentication owner |
| ------------------ | --------------------------------------------- | -------------------- |
| ChatGPT Plus / Pro | Codex through ACP or a Terminal Thread        | Codex or OpenAI      |
| Claude Pro / Max   | Claude Agent through ACP or Claude Code       | Anthropic            |
| GitHub Copilot     | Copilot agent or CLI, where available         | GitHub               |
| Cursor             | Cursor External Agent or CLI, where available | Cursor               |

Use [External Agents](./external-agents.md) for ACP integrations or
[Terminal Threads](./terminal-threads.md) for native CLI and TUI workflows.
These tools own their authentication, model routing, usage limits, and billing.

Provider API credits are separate from consumer subscriptions. When a provider
gives you an API key, follow [Use API Access](./use-api-access.md).
