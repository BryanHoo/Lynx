---
title: Local AI Edit Prediction in Lynx
description: Configure local or self-hosted edit predictions with Ollama or an OpenAI-compatible server.
---

# Edit Prediction

Lynx can request inline and multi-line code edits from a local or self-hosted model. Press `tab` to accept a prediction.

## Providers

Open the Settings Editor at `edit_predictions.providers` and configure one of these providers:

- `ollama`: an Ollama server running locally or on your network
- `open_ai_compatible_api`: a server that implements the OpenAI `/v1/completions` endpoint
- `none`: disable edit predictions

### Ollama

```json [settings]
{
  "edit_predictions": {
    "provider": "ollama",
    "ollama": {
      "api_url": "http://localhost:11434",
      "model": "qwen2.5-coder:7b-base",
      "prompt_format": "infer",
      "max_output_tokens": 512,
      "prediction_debounce": 0
    }
  }
}
```

### OpenAI-Compatible Servers

```json [settings]
{
  "edit_predictions": {
    "provider": "open_ai_compatible_api",
    "open_ai_compatible_api": {
      "api_url": "http://localhost:8080/v1/completions",
      "model": "deepseek-coder-6.7b-base",
      "prompt_format": "deepseek_coder",
      "max_output_tokens": 512,
      "prediction_debounce": 0
    }
  }
}
```

An API key configured in the provider page is sent as `Authorization: Bearer {key}`.

## Prompt Formats

Use `infer` to detect a format from the model name, or set one explicitly:

- `zeta`, `zeta2`, `zeta2_1`
- `code_llama`
- `star_coder`
- `deepseek_coder`
- `qwen`
- `code_gemma`
- `codestral`
- `glm`
- `sweep`

## Display Modes

Set `edit_predictions.mode` to:

- `eager`: display predictions inline when they do not conflict with language server completions
- `subtle`: display predictions while holding the configured modifier key

## Prediction Debounce

`prediction_debounce` controls the delay in milliseconds after typing stops. Set it to `0` to request immediately. Manually invoking {#action editor::ShowEditPrediction} bypasses this delay.

## Scope

Disable automatic predictions globally:

```json [settings]
{
  "show_edit_predictions": false
}
```

Disable them for a language:

```json [settings]
{
  "languages": {
    "Python": {
      "show_edit_predictions": false
    }
  }
}
```

Exclude sensitive paths:

```json [settings]
{
  "edit_predictions": {
    "disabled_globs": ["~/.config/zed/settings.json"]
  }
}
```

## Key Bindings

`tab` accepts a prediction in eager mode when the completion menu is closed. `alt-tab` accepts or previews predictions in all modes; Linux and Windows also bind `alt-l`.

These actions support partial acceptance:

- {#action editor::AcceptNextWordEditPrediction}
- {#action editor::AcceptNextLineEditPrediction}

## See Also

- [Agent Panel](./agent-panel.md)
- [Inline Assistant](./inline-assistant.md)
- [Use a Local Model](./use-a-local-model.md)
