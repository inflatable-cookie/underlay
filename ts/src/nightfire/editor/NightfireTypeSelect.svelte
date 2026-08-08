<script lang="ts">
  import { Select } from "@inflatable-cookie/poodle-svelte";
  import type { SelectItems, SelectOptionGroup } from "@inflatable-cookie/poodle-svelte/types";
  import type { GroupedOptions, NightfireBlockOptionInput } from "./grouped-options";

  type TypeOption = Pick<NightfireBlockOptionInput, "type" | "label">;

  interface Props {
    value?: string;
    groupedOptions?: GroupedOptions[] | null;
    typeOptions?: TypeOption[];
    onChange: (value: string) => void;
    variant?: "default" | "ghost";
    menuMinWidth?: string | null;
  }

  let {
    value,
    groupedOptions = null,
    typeOptions = [],
    onChange,
    variant = "default",
    menuMinWidth = null
  }: Props = $props();

  function formatSubgroupLabel(input: string): string {
    return input
      .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
      .replace(/[_-]/g, " ")
      .trim();
  }

  function formatOptionLabel(input: string): string {
    const trimmed = input.trim();
    const withoutQuestionPrefix = trimmed.replace(/^question\s*(?:[-:–—]\s*)?/i, "");
    return withoutQuestionPrefix.length > 0 ? withoutQuestionPrefix : trimmed;
  }

  function subcategoryPriority(input: string): number {
    const normalised = input.toLowerCase().replace(/[^a-z0-9]/g, "");
    if (normalised === "multiplechoice") return 1;
    if (normalised === "input") return 2;
    if (normalised === "interactive") return 3;
    if (normalised === "draganddrop") return 4;
    if (normalised === "other") return 5;
    return 99;
  }

  const items = $derived(
    typeOptions.map((opt) => ({ value: opt.type, label: formatOptionLabel(opt.label) }))
  );

  const options = $derived.by<SelectItems>(() => {
    if (!groupedOptions?.length) return items;

    const renderedGroups: SelectOptionGroup[] = [];

    for (const group of groupedOptions) {
      const subgroups = new Map<string, Array<{ value: string; label: string }>>();
      const directItems: Array<{ value: string; label: string }> = [];

      for (const option of group.options) {
        const item = { value: option.type, label: formatOptionLabel(option.label) };
        if (option.subcategory) {
          const existing = subgroups.get(option.subcategory) ?? [];
          existing.push(item);
          subgroups.set(option.subcategory, existing);
        } else {
          directItems.push(item);
        }
      }

      if (directItems.length) {
        renderedGroups.push({
          label: group.label,
          options: directItems.sort((a, b) => a.label.localeCompare(b.label)),
        });
      }

      const nestedGroups = Array.from(subgroups.entries())
        .sort(([a], [b]) => {
          const priorityA = subcategoryPriority(a);
          const priorityB = subcategoryPriority(b);
          if (priorityA !== priorityB) {
            return priorityA - priorityB;
          }
          return a.localeCompare(b);
        })
        .map(([subcategory, subgroupItems]) => ({
          label: `${group.label} / ${formatSubgroupLabel(subcategory)}`,
          options: subgroupItems.sort((a, b) => a.label.localeCompare(b.label)),
        }));

      renderedGroups.push(...nestedGroups);
    }

    return renderedGroups.length ? renderedGroups : items;
  });
</script>

<Select
  value={value ?? ""}
  {options}
  ariaLabel="Block type"
  {variant}
  {menuMinWidth}
  onValueChange={onChange}
/>
