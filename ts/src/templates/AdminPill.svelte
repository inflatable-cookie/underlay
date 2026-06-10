<script lang="ts">
  import { Pill } from "@poodle/svelte";
  import Eye from "lucide-svelte/icons/eye";
  import EyeOff from "lucide-svelte/icons/eye-off";
  import Lock from "lucide-svelte/icons/lock";
  import LockOpen from "lucide-svelte/icons/lock-open";
  import { ADMIN_PILL_ACCENTS, type AdminPillKind } from "./admin-pill-accents";

  interface Props {
    kind?: AdminPillKind;
    label?: string | number | null;
    accent?: string | null;
    typography?: "label" | "inherit";
    size?: "xs" | "sm" | "md" | "lg" | "xl" | null;
    preserveCase?: boolean;
  }

  let {
    kind = "neutral",
    label = null,
    accent = null,
    typography = "inherit",
    size = "sm",
    preserveCase = false
  }: Props = $props();

  const fallbackLabel = $derived(kind);
  const resolvedAccent = $derived(accent ?? ADMIN_PILL_ACCENTS[kind]);
  const resolvedLabel = $derived.by(() => {
    const raw = label ?? fallbackLabel;
    const text = String(raw).replaceAll("_", " ");
    return preserveCase ? text : text.toLowerCase();
  });
</script>

<Pill accent={resolvedAccent} {typography} {size}>
  {#if kind === "live"}
    <Eye size={14} />
  {:else if kind === "draft"}
    <EyeOff size={14} />
  {:else if kind === "free"}
    <LockOpen size={14} />
  {:else if kind === "restricted"}
    <Lock size={14} />
  {/if}
  {resolvedLabel}
</Pill>

