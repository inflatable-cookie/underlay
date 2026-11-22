<script lang="ts">
  import { onDestroy } from "svelte";
  import User from "lucide-svelte/icons/user";
  import LogOut from "lucide-svelte/icons/log-out";
  import {
    AlertDialog as PoodleAlertDialog,
    Popover as PoodlePopover
  } from "@inflatable-cookie/poodle-svelte";
  import { resolveRedirectTo } from "../client/route-protection";

  interface Props {
    displayName?: string | null;
    roles?: string[] | null;
    appName?: string;
    accountHref?: string;
    avatarGradient?: string;
    onLogout: () => Promise<void> | void;
    variant?: "desktop" | "mobile";
    onNavigate?: () => void;
  }

  // Theme token keeps the avatar gradient in step with the admin nav badges;
  // the literal is only a fallback for apps that do not load Underlay tokens.
  const DEFAULT_AVATAR_GRADIENT =
    "var(--underlay-admin-nav-badge-overview, linear-gradient(135deg, #22c55e, #14b8a6))";

  let {
    displayName = null,
    roles = null,
    appName = "Admin",
    accountHref = "/account",
    avatarGradient = DEFAULT_AVATAR_GRADIENT,
    onLogout,
    variant = "desktop",
    onNavigate
  }: Props = $props();

  let userMenuOpen = $state(false);
  let logoutOpen = $state(false);
  let loggingOut = $state(false);
  let logoutError = $state<string | null>(null);

  let disposed = false;

  onDestroy(() => {
    disposed = true;
  });

  const safeAccountHref = $derived(resolveRedirectTo(accountHref, "/account"));

  const initialsFromName = (name: string) => {
    const parts = name
      .split(/\s+/)
      .map((p) => p.trim())
      .filter(Boolean);

    if (parts.length === 0) return "??";

    const first = parts[0]?.[0] ?? "";
    const second = parts.length > 1 ? parts[1]?.[0] ?? "" : parts[0]?.[1] ?? "";
    return (first + second).toUpperCase() || "??";
  };

  const roleLabel = (roles: string[] | null | undefined) => {
    const role = roles?.[0] ?? "";
    if (!role) return "Staff";
    return role.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
  };

  const confirmLogout = async () => {
    if (loggingOut) return;
    loggingOut = true;
    logoutError = null;
    try {
      await onLogout();
    } catch (e) {
      // A rejected logout must not silently close the dialog: keep it open and
      // surface the failure so the user can retry instead of believing they
      // are signed out.
      if (disposed) return;
      logoutError =
        e instanceof Error && e.message.trim()
          ? e.message.trim()
          : "Log out failed. Please try again.";
      return;
    } finally {
      if (!disposed) loggingOut = false;
    }

    if (disposed) return;
    logoutOpen = false;
  };

  const handleNavigate = () => {
    userMenuOpen = false;
    onNavigate?.();
  };

  const handleLogoutClick = () => {
    userMenuOpen = false;
    logoutError = null;
    onNavigate?.();
    logoutOpen = true;
  };
</script>

<div class={variant === "mobile" ? "admin-mobile-overlay__user" : "admin-nav__user-section"}>
  <PoodlePopover bind:open={userMenuOpen} placement="top-start" ariaLabel="User menu" block>
    {#snippet trigger()}
      <div class="admin-nav__user-trigger" data-state={userMenuOpen ? "open" : "closed"}>
        <div class="admin-nav__user-avatar" style="background: {avatarGradient}" aria-hidden="true">
          {initialsFromName(displayName ?? "Admin")}
        </div>
        <div class="admin-nav__user-meta">
          <span class="admin-nav__user-name">{displayName ?? "Admin user"}</span>
          <span class="admin-nav__user-role">{roleLabel(roles)}</span>
        </div>
      </div>
    {/snippet}

    <nav class="admin-nav__user-menu" aria-label="User menu">
      <a href={safeAccountHref} class="admin-nav__user-menu-item" onclick={handleNavigate}>
        <User class="admin-nav__user-menu-icon" aria-hidden="true" />
        <span>Account</span>
      </a>
      <button
        type="button"
        class="admin-nav__user-menu-item admin-nav__user-menu-item--danger"
        onclick={handleLogoutClick}
        disabled={loggingOut}
      >
        <LogOut class="admin-nav__user-menu-icon" aria-hidden="true" />
        <span>Log out</span>
      </button>
    </nav>
  </PoodlePopover>

  <PoodleAlertDialog
    bind:open={logoutOpen}
    title="Log out?"
    description={logoutError ?? `You'll be signed out of ${appName}.`}
    confirmLabel={loggingOut ? "Logging out..." : "Log out"}
    cancelLabel="Cancel"
    onConfirm={confirmLogout}
    onCancel={() => (logoutOpen = false)}
    tone="danger"
  />
</div>

<style>
  .admin-nav__user-section {
    margin-top: 1rem;
    padding: 1rem 0.9rem 0 0;
    border-top: 1px solid var(--underlay-color-border-subtle);
    flex-shrink: 0;
  }

  .admin-nav__user-trigger {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.5rem;
    border-radius: var(--underlay-radius-control, 0.5rem);
    background: transparent;
    border: 1px solid transparent;
    justify-content: flex-start;
    gap: 0.6rem;
  }

  .admin-nav__user-trigger:hover {
    background: var(--underlay-color-hover-bg);
  }

  .admin-nav__user-trigger[data-state="open"] {
    background: var(--underlay-color-accent-tint-bg);
    border-color: var(--underlay-color-border-subtle);
  }

  .admin-nav__user-avatar {
    width: 2rem;
    height: 2rem;
    border-radius: var(--underlay-radius-pill, 9999px);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--underlay-font-size-xs, 0.75rem);
    font-weight: 700;
    color: var(--underlay-admin-nav-badge-fg);
    flex-shrink: 0;
  }

  .admin-nav__user-meta {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    text-align: left;
    min-width: 0;
  }

  .admin-nav__user-name {
    font-size: 0.85rem;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .admin-nav__user-role {
    font-size: 0.7rem;
    color: var(--underlay-color-text-muted);
  }

  .admin-nav__user-menu {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
    width: 100%;
    min-width: 0;
  }

  .admin-nav__user-section :global(.poodle-popover),
  .admin-mobile-overlay__user :global(.poodle-popover) {
    width: 100%;
  }

  .admin-nav__user-section :global(.poodle-popover__surface),
  .admin-mobile-overlay__user :global(.poodle-popover__surface) {
    width: 100%;
    min-width: 100%;
    box-sizing: border-box;
  }

  .admin-nav__user-menu-item {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.5rem 0.65rem;
    border-radius: var(--underlay-radius-sm, 0.4rem);
    background: transparent;
    border: none;
    color: inherit;
    font-size: 0.85rem;
    text-decoration: none;
    cursor: pointer;
    width: 100%;
    box-sizing: border-box;
    text-align: left;
  }

  .admin-nav__user-menu-item:hover {
    background: var(--underlay-color-hover-bg);
  }

  .admin-nav__user-menu-item:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-color-accent);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .admin-nav__user-menu-item--danger {
    color: var(--underlay-color-danger-strong);
  }

  .admin-nav__user-menu-item--danger:hover,
  .admin-nav__user-menu-item--danger:focus-visible {
    background: var(--underlay-color-danger-bg);
  }

  .admin-nav__user-menu-item :global(.admin-nav__user-menu-icon) {
    width: 1rem;
    height: 1rem;
    flex-shrink: 0;
  }

  .admin-mobile-overlay__user {
    margin-top: auto;
    padding-top: 1rem;
    border-top: 1px solid var(--underlay-color-border-subtle);
  }
</style>
