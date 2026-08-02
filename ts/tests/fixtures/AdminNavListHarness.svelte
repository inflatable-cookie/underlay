<script lang="ts">
  import AdminNavList from "../../src/templates/AdminNavList.svelte";
  import type { AdminNavItem } from "../../src/templates/admin-nav.types";
  import TestIcon from "./TestIcon.svelte";

  interface Props {
    currentSection?: string | null;
    currentPath?: string;
    onNavigate?: () => void;
  }

  let { currentSection = null, currentPath = "", onNavigate = undefined }: Props = $props();

  const items: AdminNavItem[] = [
    { type: "link", href: "/", label: "Overview", icon: TestIcon },
    {
      type: "section",
      id: "content",
      label: "Content",
      icon: TestIcon,
      children: [
        { href: "/content/pages", label: "Pages" },
        { href: "/content/trash", label: "Trash", danger: true },
        {
          href: "/content/archived",
          label: "Archived",
          excludeHrefs: ["/content/archived/hidden"]
        }
      ]
    },
    {
      type: "section",
      id: "system",
      label: "System",
      icon: TestIcon,
      children: [{ href: "/system/jobs", label: "Jobs" }]
    }
  ];
</script>

<AdminNavList {items} {currentSection} {currentPath} {onNavigate} />
