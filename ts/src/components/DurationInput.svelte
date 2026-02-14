<script lang="ts">
  import { untrack } from "svelte";
  import ChevronUp from "lucide-svelte/icons/chevron-up";
  import ChevronDown from "lucide-svelte/icons/chevron-down";

  interface Props {
    /**
     * Total duration as string (bindable). Empty string = no value.
     * When unit="seconds" (default): value is total seconds.
     * When unit="minutes": value is total minutes.
     */
    value?: string;
    /** Hidden input name for form submission */
    name?: string;
    /** Component ID */
    id?: string;
    /** Whether field is required */
    required?: boolean;
    /** Whether field is disabled */
    disabled?: boolean;
    /** Minimum total (in the selected unit) */
    min?: number;
    /** Maximum total (in the selected unit) */
    max?: number;
    /**
     * Unit for the value. Defaults to "seconds" (H:MM:SS).
     * Set to "minutes" to hide the seconds segment and treat value as total minutes (H:MM).
     */
    unit?: "seconds" | "minutes";
    /** Additional CSS class */
    class?: string;
  }

  let {
    value = $bindable(""),
    name,
    id,
    required = false,
    disabled = false,
    min,
    max,
    unit = "seconds",
    class: className,
  }: Props = $props();

  const showSeconds = $derived(unit === "seconds");

  // --- Decompose / Compose helpers ---
  // When unit="seconds": value is total seconds, segments are H:MM:SS
  // When unit="minutes": value is total minutes, segments are H:MM (no seconds)

  function decompose(totalStr: string): { h: string; m: string; s: string } {
    if (!totalStr) return { h: "", m: "", s: "" };
    const total = parseInt(totalStr, 10);
    if (isNaN(total) || total <= 0) return { h: "", m: "", s: "" };

    if (unit === "minutes") {
      // total is minutes → decompose into hours and minutes
      const h = Math.floor(total / 60);
      const m = total % 60;
      return {
        h: h > 0 ? String(h) : "",
        m: m > 0 || h > 0 ? String(m) : "",
        s: "",
      };
    }

    // total is seconds → decompose into hours, minutes, seconds
    const h = Math.floor(total / 3600);
    const m = Math.floor((total % 3600) / 60);
    const s = total % 60;
    return {
      h: h > 0 ? String(h) : "",
      m: m > 0 || h > 0 ? String(m) : "",
      s: String(s),
    };
  }

  function compose(h: string, m: string, s: string): string {
    const hNum = parseInt(h, 10) || 0;
    const mNum = parseInt(m, 10) || 0;
    const sNum = parseInt(s, 10) || 0;

    if (unit === "minutes") {
      const total = hNum * 60 + mNum;
      return total > 0 ? String(total) : "";
    }

    const total = hNum * 3600 + mNum * 60 + sNum;
    return total > 0 ? String(total) : "";
  }

  function clampSegment(raw: string, maxVal: number): string {
    if (!raw) return "";
    const num = parseInt(raw, 10);
    if (isNaN(num)) return "";
    if (num < 0) return "0";
    if (num > maxVal) return String(maxVal);
    return String(num);
  }

  function clampTotal(totalStr: string): string {
    if (!totalStr) return "";
    let total = parseInt(totalStr, 10);
    if (isNaN(total) || total <= 0) return "";
    if (min !== undefined && total < min) total = min;
    if (max !== undefined && total > max) total = max;
    return String(total);
  }

  // --- Internal state ---

  let hours = $state("");
  let minutes = $state("");
  let seconds = $state("");

  // Track last external value to detect external vs internal changes
  let lastExternalValue = $state(untrack(() => value));

  // Init segments from initial value
  {
    const parts = decompose(untrack(() => value));
    hours = parts.h;
    minutes = parts.m;
    seconds = parts.s;
  }

  // Sync external → internal when value changes externally
  $effect(() => {
    if (value !== lastExternalValue) {
      const parts = decompose(value);
      hours = parts.h;
      minutes = parts.m;
      seconds = parts.s;
      lastExternalValue = value;
    }
  });

  // Derived total for hidden input (seconds or minutes depending on unit)
  const totalValue = $derived(compose(hours, minutes, seconds));

  function syncToValue() {
    const newTotal = compose(hours, minutes, seconds);
    lastExternalValue = newTotal;
    value = newTotal;
  }

  // --- Segment refs ---

  let hoursRef = $state<HTMLInputElement | null>(null);
  let minutesRef = $state<HTMLInputElement | null>(null);
  let secondsRef = $state<HTMLInputElement | null>(null);

  // Track which segment is focused and last-focused for stepper context
  type Segment = "h" | "m" | "s";
  let focusedSegment = $state<Segment | null>(null);
  let lastFocusedSegment = $state<Segment>(untrack(() => showSeconds) ? "s" : "m");

  // --- Input handling ---

  function handleSegmentInput(segment: Segment, event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    const sanitized = target.value.replace(/[^0-9]/g, "").slice(0, 2);
    if (segment === "h") hours = sanitized;
    else if (segment === "m") minutes = sanitized;
    else seconds = sanitized;
    // Update the actual input value to the sanitized version
    target.value = sanitized;
    syncToValue();
  }

  function handleBlur(segment: Segment) {
    // Small delay to let focus move to another segment before clearing
    setTimeout(() => {
      // Only clear focusedSegment if nothing in this component has focus
      const active = document.activeElement;
      if (
        active !== hoursRef &&
        active !== minutesRef &&
        (active !== secondsRef || !showSeconds)
      ) {
        focusedSegment = null;
      }
    }, 0);

    // Clamp individual segment
    if (segment === "h") hours = clampSegment(hours, 99);
    else if (segment === "m") minutes = clampSegment(minutes, 59);
    else seconds = clampSegment(seconds, 59);

    // Compose, clamp total, then re-decompose to normalise
    const newTotal = clampTotal(compose(hours, minutes, seconds));
    const parts = decompose(newTotal);
    hours = parts.h;
    minutes = parts.m;
    seconds = parts.s;
    lastExternalValue = newTotal;
    value = newTotal;
  }

  function handleFocus(segment: Segment) {
    focusedSegment = segment;
    lastFocusedSegment = segment;
  }

  function handleKeydown(segment: Segment, event: KeyboardEvent) {
    if (event.key === "ArrowUp") {
      event.preventDefault();
      incrementSegment(segment);
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      decrementSegment(segment);
    }
  }

  function incrementSegment(segment: Segment) {
    if (disabled) return;
    const maxVal = segment === "h" ? 99 : 59;
    const current = parseInt(
      segment === "h" ? hours : segment === "m" ? minutes : seconds,
      10
    ) || 0;
    const next = Math.min(current + 1, maxVal);
    if (segment === "h") hours = String(next);
    else if (segment === "m") minutes = String(next);
    else seconds = String(next);
    syncToValue();
  }

  function decrementSegment(segment: Segment) {
    if (disabled) return;
    const current = parseInt(
      segment === "h" ? hours : segment === "m" ? minutes : seconds,
      10
    ) || 0;
    const next = Math.max(current - 1, 0);
    if (segment === "h") hours = String(next);
    else if (segment === "m") minutes = String(next);
    else seconds = String(next);
    syncToValue();
  }

  // Stepper operates on focused segment, or last-focused, or seconds by default
  function incrementFocused() {
    const seg = focusedSegment ?? lastFocusedSegment;
    incrementSegment(seg);
    refForSegment(seg)?.focus();
  }

  function decrementFocused() {
    const seg = focusedSegment ?? lastFocusedSegment;
    decrementSegment(seg);
    refForSegment(seg)?.focus();
  }

  function refForSegment(seg: Segment): HTMLInputElement | null {
    if (seg === "h") return hoursRef;
    if (seg === "m") return minutesRef;
    if (seg === "s" && showSeconds) return secondsRef;
    return minutesRef;
  }
</script>

{#if name}
  <input type="hidden" {name} value={totalValue} />
{/if}

<div
  class="underlay-duration-input {className ?? ''}"
  class:underlay-duration-input--disabled={disabled}
  class:underlay-duration-input--focused={focusedSegment !== null}
  {id}
  role="group"
  aria-label="Duration"
>
  <div class="underlay-duration-input__fields">
    <!-- Hours -->
    <div class="underlay-duration-input__segment">
      <input
        bind:this={hoursRef}
        type="text"
        inputmode="numeric"
        pattern="[0-9]*"
        class="underlay-duration-input__input"
        value={hours}
        placeholder="0"
        maxlength={2}
        disabled={disabled}
        autocomplete="off"
        aria-label="Hours"
        oninput={(e) => handleSegmentInput("h", e)}
        onblur={() => handleBlur("h")}
        onfocus={() => handleFocus("h")}
        onkeydown={(e) => handleKeydown("h", e)}
      /><span class="underlay-duration-input__unit">h</span>
    </div>

    <span class="underlay-duration-input__separator">:</span>

    <!-- Minutes -->
    <div class="underlay-duration-input__segment">
      <input
        bind:this={minutesRef}
        type="text"
        inputmode="numeric"
        pattern="[0-9]*"
        class="underlay-duration-input__input"
        value={minutes}
        placeholder="0"
        maxlength={2}
        disabled={disabled}
        autocomplete="off"
        aria-label="Minutes"
        oninput={(e) => handleSegmentInput("m", e)}
        onblur={() => handleBlur("m")}
        onfocus={() => handleFocus("m")}
        onkeydown={(e) => handleKeydown("m", e)}
      /><span class="underlay-duration-input__unit">m</span>
    </div>

    {#if showSeconds}
      <span class="underlay-duration-input__separator">:</span>

      <!-- Seconds -->
      <div class="underlay-duration-input__segment">
        <input
          bind:this={secondsRef}
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          class="underlay-duration-input__input"
          value={seconds}
          placeholder="0"
          maxlength={2}
          disabled={disabled}
          autocomplete="off"
          aria-label="Seconds"
          oninput={(e) => handleSegmentInput("s", e)}
          onblur={() => handleBlur("s")}
          onfocus={() => handleFocus("s")}
          onkeydown={(e) => handleKeydown("s", e)}
        /><span class="underlay-duration-input__unit">s</span>
      </div>
    {/if}
  </div>

  <!-- Single stepper controls on right edge -->
  <div class="underlay-duration-input__controls">
    <button
      type="button"
      class="underlay-duration-input__button underlay-duration-input__button--up"
      onclick={incrementFocused}
      disabled={disabled}
      tabindex={-1}
      aria-label="Increment"
    >
      <ChevronUp size="1em" strokeWidth={2.5} />
    </button>
    <button
      type="button"
      class="underlay-duration-input__button underlay-duration-input__button--down"
      onclick={decrementFocused}
      disabled={disabled}
      tabindex={-1}
      aria-label="Decrement"
    >
      <ChevronDown size="1em" strokeWidth={2.5} />
    </button>
  </div>
</div>

<style>
  .underlay-duration-input {
    display: inline-flex;
    align-items: stretch;
    width: max-content;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    border-radius: var(--underlay-radius-sm, 0.35rem);
    transition: outline-color 0.15s ease;
    outline: 2px solid transparent;
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-duration-input--focused {
    outline-color: var(--underlay-color-primary, #2563eb);
  }

  .underlay-duration-input__fields {
    display: flex;
    align-items: center;
    gap: 0.15em;
    padding: 0.2em 0.25em;
  }

  .underlay-duration-input__segment {
    display: flex;
    align-items: center;
    background: rgba(0, 0, 0, 0.15);
    border-radius: calc(var(--underlay-radius-sm, 0.35rem) * 0.6);
    padding-right: 0.25em;
  }

  .underlay-duration-input__input {
    width: 2ch;
    box-sizing: content-box;
    padding: 0.3em 0.15em 0.3em 0.3em;
    border: none;
    background: transparent;
    color: var(--underlay-color-text, #e5e7eb);
    font-size: inherit;
    font-family: inherit;
    text-align: center;
    outline: none;
  }

  .underlay-duration-input__input::placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.5;
  }

  .underlay-duration-input__unit {
    font-size: 0.7em;
    color: var(--underlay-color-text-muted, #9ca3af);
    user-select: none;
    pointer-events: none;
    line-height: 1;
  }

  .underlay-duration-input__separator {
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.7;
    font-weight: 500;
    font-size: 0.85em;
    user-select: none;
    padding: 0 0.05em;
  }

  .underlay-duration-input__controls {
    display: flex;
    flex-direction: column;
    width: 1.2em;
    border-left: 1px solid rgba(148, 163, 184, 0.12);
    border-radius: 0 var(--underlay-radius-sm, 0.35rem) var(--underlay-radius-sm, 0.35rem) 0;
    overflow: hidden;
  }

  .underlay-duration-input__button {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
    font-size: 0.7em;
  }

  .underlay-duration-input__button:hover:not(:disabled) {
    background: var(--underlay-color-primary-subtle, rgba(37, 99, 235, 0.15));
    color: var(--underlay-color-primary, #2563eb);
  }

  .underlay-duration-input__button:active:not(:disabled) {
    background: var(--underlay-color-primary-subtle, rgba(37, 99, 235, 0.25));
  }

  .underlay-duration-input__button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .underlay-duration-input__button--up {
    border-bottom: 1px solid rgba(148, 163, 184, 0.12);
  }

  .underlay-duration-input--disabled {
    opacity: 0.6;
    pointer-events: none;
  }
</style>
