<script lang="ts">
  import Select from "../../components/Select.svelte";
  import type { GroupedOptions, NightfireBlockOptionInput } from "./grouped-options";

  type TypeOption = Pick<NightfireBlockOptionInput, "type" | "label">;
  type SelectItem = { value: string; label: string; disabled?: boolean };
  type SelectGroup = { label: string; items?: SelectItem[]; groups?: SelectGroup[] };

  interface Props {
    value?: string;
    groupedOptions?: GroupedOptions[] | null;
    typeOptions?: TypeOption[];
    onChange: (value: string) => void;
  }

  let {
    value,
    groupedOptions = null,
    typeOptions = [],
    onChange
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

  const groups = $derived.by<SelectGroup[] | null>(() => {
    if (!groupedOptions?.length) return null;

    return groupedOptions.map((group) => {
      const subgroups = new Map<string, SelectItem[]>();
      const directItems: SelectItem[] = [];

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

      const nestedGroups: SelectGroup[] = Array.from(subgroups.entries())
        .sort(([a], [b]) => {
          const priorityA = subcategoryPriority(a);
          const priorityB = subcategoryPriority(b);
          if (priorityA !== priorityB) {
            return priorityA - priorityB;
          }
          return a.localeCompare(b);
        })
        .map(([subcategory, subgroupItems]) => ({
          label: formatSubgroupLabel(subcategory),
          items: subgroupItems.sort((a, b) => a.label.localeCompare(b.label))
        }));

      return {
        label: group.label,
        items: directItems.length ? directItems.sort((a, b) => a.label.localeCompare(b.label)) : undefined,
        groups: nestedGroups.length ? nestedGroups : undefined
      };
    });
  });
</script>

<Select
  value={value ?? ""}
  items={groups ? undefined : items}
  {groups}
  triggerAriaLabel="Block type"
  onchange={onChange}
/>
