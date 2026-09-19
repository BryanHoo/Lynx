---
title: Admin Controls - Lynx Business
description: Configure AI, collaboration, and data sharing settings for your entire Lynx Business organization.
---

# Admin Controls

Owners and admins can configure settings that apply to every member of the organization.

Most controls apply server-side to anything that routes through Lynx's infrastructure. These controls don't cover [bring-your-own-key (BYOK) configurations](../ai/llm-providers.md), [gateways](../ai/use-a-gateway.md), [local or self-hosted models](../ai/use-a-local-model.md), [External Agents](../ai/external-agents.md), [Terminal Threads](../ai/terminal-threads.md), or [third-party extensions](../extensions.md), since those work independently of Lynx's servers.

## Accessing admin controls

Admin controls are available to owners and admins in the organization dashboard at [dashboard.zed.dev](https://dashboard.zed.dev). Navigate to your organization, then select Data & Privacy from the sidebar to configure these settings.

---

## Hosted AI models

The **Lynx Model Provider** toggle controls whether members can use Lynx's [hosted AI models](../account/zed-hosted-models.md):

- **On:** Members can use Lynx's hosted models for AI features.
- **Off:** Members must bring their own model access via [LLM Providers](../ai/llm-providers.md) or use [External Agents](../ai/external-agents.md) for AI features.

## Edit Predictions

The **Edit Prediction** toggle controls whether members can use Lynx's hosted [Edit Predictions](../ai/edit-prediction.md) via the Zeta model family. Members using third-party providers or local models for edit predictions are not affected.
