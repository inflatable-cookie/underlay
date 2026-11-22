<script lang="ts">
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import type { AdminNavChild, AdminNavItem } from "./admin-nav.types";

  interface Props {
    items: AdminNavItem[];
    currentSection?: string | null;
    currentPath?: string;
    onNavigate?: () => void;
    variant?: "desktop" | "mobile";
  }

  let {
    items,
    currentSection = null,
    currentPath = "",
    onNavigate,
    variant = "desktop"
  }: Props = $props();

  const uid = $props.id();
  let expandedSection = $state<string | null>(null);

  $effect(() => {
    if (currentSection) {
      expandedSection = currentSection;
    }
  });

  const handleChildClick = () => {
    onNavigate?.();
  };

  const toggleSection = (section: string) => {
    expandedSection = expandedSection === section ? null : section;
  };

  function sectionListId(sectionId: string): string {
    return `${uid}-section-${sectionId}`;
  }

  function isActive(href: string): boolean {
    if (!currentPath) return false;
    if (href === "/") return currentPath === "/";
    return currentPath === href || currentPath.startsWith(href + "/");
  }

  function isChildActive(child: AdminNavChild): boolean {
    if (!isActive(child.href)) return false;
    return !(child.excludeHrefs ?? []).some((excluded) => isActive(excluded));
  }

  const isMobile = $derived(variant === "mobile");
</script>

<ul class={isMobile ? "admin-mobile-overlay__list" : "admin-nav__list"}>
  {#each items as item (item.type === "section" ? `section:${item.id}` : `link:${item.href}`)}
    <li>
      {#if item.type === "link"}
        {@const active = isActive(item.href)}
        <a
          href={item.href}
          class="admin-nav__root"
          class:admin-mobile-overlay__link={isMobile}
          class:admin-nav__link--active={active}
          aria-current={active ? "page" : undefined}
          onclick={handleChildClick}
        >
          <span
            class="admin-nav__badge {item.badgeClass ?? ''}"
            style={item.badgeGradient ? `background: ${item.badgeGradient}` : undefined}
            aria-hidden="true"
          >
            <item.icon class="admin-nav__badge-icon" />
          </span>
          <span class:admin-nav__label={!isMobile}>{item.label}</span>
        </a>
      {:else}
        {@const controlsId = sectionListId(item.id)}
        {@const isExpanded = expandedSection === item.id}
        <button
          type="button"
          class="admin-nav__section-toggle"
          class:admin-nav__root={!isMobile}
          class:admin-nav__section={!isMobile}
          class:admin-mobile-overlay__link={isMobile}
          aria-expanded={isExpanded}
          aria-controls={isExpanded ? controlsId : undefined}
          onclick={() => toggleSection(item.id)}
        >
          <span
            class="admin-nav__badge {item.badgeClass ?? ''}"
            style={item.badgeGradient ? `background: ${item.badgeGradient}` : undefined}
            aria-hidden="true"
          >
            <item.icon class="admin-nav__badge-icon" />
          </span>
          <span class:admin-nav__label={!isMobile}>{item.label}</span>
          <ChevronDown class="admin-nav__chevron" aria-hidden="true" />
        </button>
        {#if expandedSection === item.id}
          <ul
            id={controlsId}
            class={isMobile ? "admin-mobile-overlay__children" : "admin-nav__children"}
          >
            {#each item.children as child (`${child.href}:${child.label}`)}
              {@const childActive = isChildActive(child)}
              <li>
                <a
                  href={child.href}
                  class={child.danger ? "admin-nav__child-link--danger" : undefined}
                  class:admin-nav__link--active={childActive}
                  aria-current={childActive ? "page" : undefined}
                  onclick={handleChildClick}
                >
                  {#if child.icon}
                    <child.icon class="admin-nav__child-icon" aria-hidden="true" />
                  {/if}
                  {child.label}
                </a>
              </li>
            {/each}
          </ul>
        {/if}
      {/if}
    </li>
  {/each}
</ul>

<style>
  .admin-nav__list {
    list-style: none;
    padding: 0 0.9rem 0 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
    flex: 1;
    overflow-y: auto;
    min-height: 0;
  }

  .admin-nav__root,
  .admin-nav__section {
    display: flex;
    align-items: center;
    box-sizing: border-box;
    padding: 0.4rem 0.7rem;
    border-radius: var(--underlay-radius-sm, 0.4rem);
    color: inherit;
    text-decoration: none;
    font-size: var(--underlay-font-size-md, 0.9rem);
    border: none;
    background: transparent;
    width: 100%;
    cursor: pointer;
    text-align: left;
  }

  .admin-nav__root:hover,
  .admin-nav__section-toggle:hover {
    background-color: var(--underlay-color-hover-bg);
  }

  .admin-nav__root:focus-visible,
  .admin-nav__section-toggle:focus-visible,
  .admin-nav__children a:focus-visible,
  .admin-mobile-overlay__link:focus-visible,
  .admin-mobile-overlay__children a:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-color-accent);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
    opacity: 1;
  }

  .admin-nav__section-toggle {
    justify-content: flex-start;
    opacity: 0.7;
    transition: opacity 0.15s ease;
  }

  .admin-nav__section-toggle:hover,
  .admin-nav__section-toggle[aria-expanded="true"] {
    opacity: 1;
  }

  .admin-nav__section-toggle :global(.admin-nav__chevron) {
    width: 1rem;
    height: 1rem;
    margin-left: auto;
    opacity: 0.5;
    transition: transform 0.15s ease;
    flex-shrink: 0;
  }

  .admin-nav__section-toggle[aria-expanded="true"] :global(.admin-nav__chevron) {
    transform: rotate(180deg);
  }

  .admin-nav__children {
    list-style: none;
    padding-left: var(--underlay-space-5, 1.25rem);
    margin: 0.25rem 0 0.4rem;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .admin-nav__children a {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    font-size: 0.85rem;
    padding: 0.45rem 0.75rem;
    border-radius: var(--underlay-radius-sm, 0.4rem);
    opacity: 0.75;
    transition: opacity 0.15s ease;
    color: inherit;
    text-decoration: none;
  }

  .admin-nav__children a:hover,
  .admin-nav__children a.admin-nav__link--active {
    opacity: 1;
    background-color: var(--underlay-color-hover-bg);
  }

  .admin-nav__children a :global(.admin-nav__child-icon),
  .admin-mobile-overlay__children a :global(.admin-nav__child-icon) {
    width: 0.9rem;
    height: 0.9rem;
    flex-shrink: 0;
  }

  .admin-nav__children a.admin-nav__child-link--danger {
    color: var(--underlay-color-danger-strong);
    opacity: 1;
  }

  .admin-nav__children a.admin-nav__child-link--danger:hover,
  .admin-nav__children a.admin-nav__child-link--danger:focus-visible,
  .admin-nav__children a.admin-nav__child-link--danger.admin-nav__link--active {
    color: var(--underlay-color-danger-strong);
    opacity: 1;
    background-color: var(--underlay-color-danger-bg);
  }

  .admin-nav__link--active {
    background-color: var(--underlay-color-accent-tint-bg);
    font-weight: 550;
  }

  .admin-nav__badge {
    min-width: 1.5rem;
    min-height: 1.5rem;
    padding: 0.05rem;
    border-radius: 0.45rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    margin-right: 0.55rem;
  }

  .admin-nav__badge :global(.admin-nav__badge-icon) {
    width: 0.95rem;
    height: 0.95rem;
    color: var(--underlay-admin-nav-badge-fg);
  }

  .admin-nav__badge:global(.admin-nav__badge--overview) {
    background: var(--underlay-admin-nav-badge-overview);
  }

  .admin-nav__badge:global(.admin-nav__badge--users) {
    background: var(--underlay-admin-nav-badge-users);
  }

  .admin-nav__badge:global(.admin-nav__badge--media) {
    background: var(--underlay-admin-nav-badge-media);
  }

  .admin-nav__badge:global(.admin-nav__badge--system) {
    background: var(--underlay-admin-nav-badge-system);
  }

  .admin-mobile-overlay__list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
    flex: 1;
  }

  .admin-mobile-overlay__link {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 0.55rem;
    padding: 0.6rem 0.75rem;
    border-radius: var(--underlay-radius-control, 0.5rem);
    color: inherit;
    text-decoration: none;
    font-size: 1rem;
    border: none;
    background: transparent;
    width: 100%;
    cursor: pointer;
    text-align: left;
    opacity: 0.75;
    transition: opacity 0.15s ease;
  }

  .admin-mobile-overlay__link:hover,
  .admin-mobile-overlay__link.admin-nav__link--active,
  .admin-mobile-overlay__link[aria-expanded="true"] {
    opacity: 1;
  }

  .admin-mobile-overlay__link:hover {
    background: var(--underlay-color-hover-bg);
  }

  .admin-mobile-overlay__children {
    list-style: none;
    padding: 0.25rem 0 0.5rem 2.5rem;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .admin-mobile-overlay__children a {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    padding: 0.5rem 0.75rem;
    border-radius: var(--underlay-radius-sm, 0.4rem);
    color: inherit;
    text-decoration: none;
    font-size: 0.95rem;
    opacity: 0.75;
    transition: opacity 0.15s ease;
  }

  .admin-mobile-overlay__children a:hover,
  .admin-mobile-overlay__children a.admin-nav__link--active {
    background: var(--underlay-color-hover-bg);
    opacity: 1;
  }

  .admin-mobile-overlay__children a.admin-nav__child-link--danger {
    color: var(--underlay-color-danger-strong);
    opacity: 1;
  }

  .admin-mobile-overlay__children a.admin-nav__child-link--danger:hover,
  .admin-mobile-overlay__children a.admin-nav__child-link--danger:focus-visible,
  .admin-mobile-overlay__children a.admin-nav__child-link--danger.admin-nav__link--active {
    color: var(--underlay-color-danger-strong);
    opacity: 1;
    background: var(--underlay-color-danger-bg);
  }

  .admin-mobile-overlay__link :global(.admin-nav__chevron) {
    width: 1.25rem;
    height: 1.25rem;
  }

  .admin-nav__label {
    line-height: 1.2;
  }
</style>
