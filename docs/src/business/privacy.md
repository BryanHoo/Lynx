---
title: Privacy for Business - Lynx Business
description: How Lynx Business handles data privacy across your organization.
---

# Privacy for Business

Lynx Business applies organization-wide privacy controls. Administrators can
adjust them from [Admin Controls](./admin-controls.md).

## What's enforced by default

For all members of a Lynx Business organization:

- **No prompt sharing:** Conversations and prompts are not submitted as feedback.
- **No training data sharing:** Code context is not submitted for model training.

These protections are enforced server-side and apply to all org members.

## What data still leaves the organization

These controls cover what Lynx stores and trains on. They don't change how AI inference works: when members use Lynx's hosted models, prompts and code context are still sent to the relevant provider (Anthropic, OpenAI, Google, etc.) to generate responses. Lynx maintains no-training commitments with these providers, and zero-data-retention commitments for all models except [provider-designated models with safety retention](../ai/privacy-and-security.md#provider-safety-retention), such as Anthropic's Covered Models. See [AI Privacy](../ai/privacy-and-security.md#data-retention-and-training) for details.

[Bring-your-own-key](../ai/llm-providers.md), [gateways](../ai/use-a-gateway.md), [local or self-hosted models](../ai/use-a-local-model.md), [External Agents](../ai/external-agents.md), and [Terminal Threads](../ai/terminal-threads.md) are subject to each provider, gateway, server, agent, or CLI's own terms.

## Additional admin controls

Administrators have additional options in [Admin Controls](./admin-controls.md):

- Disable Lynx-hosted models entirely via the Lynx Model Provider toggle, so no
  prompts reach Lynx's infrastructure
- Disable Edit Predictions org-wide
- Disable real-time collaboration

See [Admin Controls](./admin-controls.md) for the full list.
