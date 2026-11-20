import { getBackButtonInfo } from "../patterns/navigation";

export function useTemplateBackInfo(
	getOptions: () => {
		href?: string | null;
		label?: string;
		contextual: boolean;
		resolveContext: boolean;
	},
) {
	let resolved = $state<{ href: string; label: string; contextual: boolean } | null>(null);

	$effect(() => {
		const { href, label = "Back", contextual, resolveContext } = getOptions();
		if (!href) {
			resolved = null;
		} else if (!resolveContext) {
			resolved = { href, label, contextual };
		} else {
			const info = getBackButtonInfo(label, href);
			resolved = {
				href: info.href,
				label: info.label,
				contextual: Boolean(info.isContextual || contextual),
			};
		}
	});

	return () => resolved;
}
