<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { parseEmbed, type ParsedEmbed, type EmbedParserConfig } from "../embed/index.js";
  import Pill from "./Pill.svelte";

  interface Props extends Omit<HTMLTextareaAttributes, "value" | "oninput" | "onchange"> {
    /** Raw input value (embed code, URL, or ID) */
    value?: string;
    /** Parsed embed result (bindable) */
    parsed?: ParsedEmbed | null;
    /** Allowed providers (empty = all) */
    providers?: string[];
    /** Allow generic/unknown embeds */
    allowGeneric?: boolean;
    /** Allowed domains for generic embeds */
    allowedDomains?: string[];
    /** Callback when input changes */
    oninput?: (value: string) => void;
    /** Callback when input loses focus */
    onchange?: (value: string) => void;
    /** Callback when parsing completes */
    onparse?: (result: ParsedEmbed | null, error?: string) => void;
    /** Debounce delay for parsing in ms (default: 300) */
    parseDebounce?: number;
    /** Parse on input (default: false, only on blur) */
    parseOnInput?: boolean;
    /** Show provider badge when detected */
    showProvider?: boolean;
  }

  let {
    value = $bindable(""),
    parsed = $bindable(null),
    providers,
    allowGeneric = true,
    allowedDomains,
    oninput,
    onchange,
    onparse,
    parseDebounce = 300,
    parseOnInput = false,
    showProvider = true,
    class: className,
    placeholder = "Paste embed code, URL, or ID...",
    rows = 3,
    ...restProps
  }: Props = $props();

  let parseError = $state<string | null>(null);
  let parseTimer: ReturnType<typeof setTimeout> | null = null;

  // Provider colors for badges
  const providerColors: Record<string, string> = {
    youtube: "#ff0000",
    vimeo: "#1ab7ea",
    audioboom: "#f55b23",
    generic: "#94a3b8",
  };

  // Cleanup timer on unmount
  $effect(() => {
    return () => {
      if (parseTimer) {
        clearTimeout(parseTimer);
      }
    };
  });

  function doParse(input: string) {
    if (!input.trim()) {
      parsed = null;
      parseError = null;
      onparse?.(null);
      return;
    }

    const config: EmbedParserConfig = {
      allowedProviders: providers,
      allowGeneric,
      allowedDomains,
    };

    const result = parseEmbed(input, config);

    if (result.success && result.parsed) {
      parsed = result.parsed;
      parseError = null;
      onparse?.(result.parsed);
    } else {
      parsed = null;
      parseError = result.error || "Could not parse embed";
      onparse?.(null, parseError);
    }
  }

  function scheduleParse(input: string) {
    if (parseTimer) {
      clearTimeout(parseTimer);
    }

    parseTimer = setTimeout(() => {
      doParse(input);
    }, parseDebounce);
  }

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLTextAreaElement | null;
    const next = target ? target.value : value;
    value = next;
    oninput?.(next);

    if (parseOnInput) {
      scheduleParse(next);
    }
  }

  function handleChange() {
    onchange?.(value);
    // Always parse on blur
    doParse(value);
  }

  function handlePaste() {
    // Parse immediately after paste
    setTimeout(() => {
      doParse(value);
    }, 0);
  }
</script>

<div class="underlay-embed-input">
  <div class="underlay-embed-input__wrapper">
    <textarea
      {...restProps}
      class={`underlay-embed-input__textarea ${parseError ? "underlay-embed-input__textarea--error" : ""} ${parsed ? "underlay-embed-input__textarea--valid" : ""} ${className ?? ""}`}
      {placeholder}
      {rows}
      bind:value
      oninput={handleInput}
      onchange={handleChange}
      onpaste={handlePaste}
    ></textarea>

    {#if showProvider && parsed}
      <div class="underlay-embed-input__badge">
        <Pill accent={providerColors[parsed.provider] ?? providerColors.generic}>
          {parsed.provider}
        </Pill>
      </div>
    {/if}
  </div>

  {#if parseError}
    <p class="underlay-embed-input__error">{parseError}</p>
  {:else if parsed}
    <p class="underlay-embed-input__success">
      {#if parsed.provider === "generic"}
        Embed detected
      {:else}
        {parsed.provider} ID: {parsed.id}
      {/if}
    </p>
  {/if}
</div>

<style>
  .underlay-embed-input {
    width: 100%;
  }

  .underlay-embed-input__wrapper {
    position: relative;
    width: 100%;
  }

  .underlay-embed-input__textarea {
    width: 100%;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, 0.55em)
      var(--underlay-field-padding-inline, 0.7em);
    border-radius: var(--underlay-radius-sm, 0.35rem);
    border: 1px solid transparent;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, inherit);
    font-size: var(--underlay-font-size-md, 0.85rem);
    font-family: var(--underlay-font-mono, monospace);
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
    resize: vertical;
    transition: border-color 0.15s ease;
  }

  .underlay-embed-input__textarea:focus,
  .underlay-embed-input__textarea:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-embed-input__textarea--error {
    border-color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-embed-input__textarea--valid {
    border-color: var(--underlay-color-success, #22c55e);
  }

  .underlay-embed-input__badge {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    pointer-events: none;
  }

  .underlay-embed-input__error {
    margin: 0.4rem 0 0;
    font-size: 0.8rem;
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-embed-input__success {
    margin: 0.4rem 0 0;
    font-size: 0.8rem;
    color: var(--underlay-color-success, #22c55e);
  }
</style>
