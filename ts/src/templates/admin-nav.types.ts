import type { Component, ComponentType } from "svelte";

export type AdminNavIcon = Component | ComponentType;

export interface AdminNavChild {
  href: string;
  label: string;
  icon?: AdminNavIcon;
  danger?: boolean;
  excludeHrefs?: string[];
}

export interface AdminNavSectionItem {
  type: "section";
  id: string;
  label: string;
  icon: AdminNavIcon;
  badgeClass?: string;
  badgeGradient?: string;
  children: AdminNavChild[];
}

export interface AdminNavLinkItem {
  type: "link";
  href: string;
  label: string;
  icon: AdminNavIcon;
  badgeClass?: string;
  badgeGradient?: string;
}

export type AdminNavItem = AdminNavSectionItem | AdminNavLinkItem;
