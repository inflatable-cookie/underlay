export const SUMMARY_SCHEMA_ID = "acow:content/summary@1";

const SUMMARY_TEXT_PAGE_TYPES = new Set([
	"summary.book",
	"summary.circles",
	"summary.pie",
	"summary.steps"
]);
const SUMMARY_IMAGE_PAGE_TYPES = new Set([
	"summary.diagram",
	"summary.slideshow"
]);
const SUMMARY_IMAGE_SLIDER_TYPE = "summary.imageSlider";

function clonePagesWithTitleBody(source: unknown): Record<string, unknown>[] {
	const record = source as Record<string, unknown> | null;
	const pages = Array.isArray(record?.pages) ? (record.pages as unknown[]) : [];
	return pages.map((page) => {
		const pageRecord = page as Record<string, unknown> | null;
		const title = pageRecord?.title;
		const body = pageRecord?.body;
		return {
			title: typeof title === "string" && title.length > 0 ? title : null,
			body: typeof body === "string" && body.length > 0 ? body : null
		};
	});
}

function hasAnyImageIds(source: unknown): boolean {
	const record = source as Record<string, unknown> | null;
	const pages = Array.isArray(record?.pages) ? (record.pages as unknown[]) : [];
	return pages.some((page) => {
		const pageRecord = page as Record<string, unknown> | null;
		return (
			typeof pageRecord?.image_id === "string" &&
			(pageRecord.image_id as string).trim().length > 0
		);
	});
}

export function transformSummaryBlockOnLayoutChange(
	currentBlock: Record<string, unknown> | null | undefined,
	nextType: string,
	getLabelForType: (type: string) => string
): { block: Record<string, unknown>; warning: string | null } {
	const fromType = typeof currentBlock?.type === "string" ? (currentBlock.type as string) : null;

	if (!fromType || fromType === nextType) {
		return {
			block: {
				...(currentBlock ?? {}),
				type: nextType
			},
			warning: null
		};
	}

	const fromIsTextPages = SUMMARY_TEXT_PAGE_TYPES.has(fromType);
	const toIsTextPages = SUMMARY_TEXT_PAGE_TYPES.has(nextType);
	const fromIsImagePages = SUMMARY_IMAGE_PAGE_TYPES.has(fromType);
	const toIsImagePages = SUMMARY_IMAGE_PAGE_TYPES.has(nextType);
	const fromIsSlider = fromType === SUMMARY_IMAGE_SLIDER_TYPE;
	const toIsSlider = nextType === SUMMARY_IMAGE_SLIDER_TYPE;

	const fromLabel = getLabelForType(fromType);
	const toLabel = getLabelForType(nextType);

	let warning: string | null = null;
	let data: Record<string, unknown> =
		currentBlock && typeof currentBlock.data === "object"
			? (currentBlock.data as Record<string, unknown>)
			: {};

	if (fromIsTextPages && toIsTextPages) {
		const pages = clonePagesWithTitleBody({ pages: data.pages });
		const subTitle =
			typeof data.subTitle === "string" && (data.subTitle as string).length > 0
				? data.subTitle
				: null;

		data = { ...data, pages };

		if (
			nextType === "summary.circles" ||
			nextType === "summary.pie" ||
			nextType === "summary.steps"
		) {
			data.subTitle = subTitle;
		} else {
			delete data.subTitle;
		}

		return {
			block: { ...(currentBlock ?? {}), type: nextType, data },
			warning: null
		};
	}

	if (fromIsImagePages && toIsImagePages) {
		return {
			block: { ...(currentBlock ?? {}), type: nextType },
			warning: null
		};
	}

	if (fromIsImagePages && toIsTextPages) {
		const pages = clonePagesWithTitleBody({ pages: data.pages });
		const hadImages = hasAnyImageIds(data);

		data = { ...data, pages };

		if (
			nextType === "summary.circles" ||
			nextType === "summary.pie" ||
			nextType === "summary.steps"
		) {
			data.subTitle =
				typeof data.subTitle === "string" && (data.subTitle as string).length > 0
					? data.subTitle
					: null;
		} else {
			delete data.subTitle;
		}

		if (hadImages) {
			warning = `Changing layout from ${fromLabel} to ${toLabel} keeps titles and bodies but drops image selections.`;
		}

		return {
			block: { ...(currentBlock ?? {}), type: nextType, data },
			warning
		};
	}

	if (fromIsTextPages && toIsImagePages) {
		const pages = clonePagesWithTitleBody({ pages: data.pages }).map((page) => ({
			...page,
			image_id: null
		}));

		data = { ...data, pages };
		return {
			block: { ...(currentBlock ?? {}), type: nextType, data },
			warning: null
		};
	}

	if ((fromIsTextPages || fromIsImagePages) && toIsSlider) {
		const pagesArray = Array.isArray(data.pages) ? (data.pages as Record<string, unknown>[]) : [];
		const first = pagesArray[0] ?? {};
		const description =
			typeof first.body === "string" && (first.body as string).length > 0
				? first.body
				: "";

		data = {
			subTitle:
				typeof data.subTitle === "string" && (data.subTitle as string).length > 0
					? data.subTitle
					: null,
			description: description || null,
			image1Id: null,
			image1Alt: null,
			image2Id: null,
			image2Alt: null,
			startPosition: "left"
		};

		if (pagesArray.length > 1 || hasAnyImageIds(currentBlock?.data)) {
			warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the first page's text as the slider description but discards other pages and any image selections.`;
		} else {
			warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the first page's text as the slider description.`;
		}

		return {
			block: { ...(currentBlock ?? {}), type: nextType, data },
			warning
		};
	}

	if (fromIsSlider && (toIsTextPages || toIsImagePages)) {
		const description =
			typeof data.description === "string" && (data.description as string).length > 0
				? data.description
				: "";

		const page = {
			title: null,
			body: description || null
		};

		data = { pages: [page] };
		if (toIsImagePages) {
			data.pages = [{ ...page, image_id: null }];
		}

		const hadImages =
			(typeof data.image1Id === "string" && (data.image1Id as string).length > 0) ||
			(typeof data.image2Id === "string" && (data.image2Id as string).length > 0);

		if (hadImages) {
			warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the description as the first page's body but drops image selections.`;
		} else {
			warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the description as the first page's body.`;
		}

		return {
			block: { ...(currentBlock ?? {}), type: nextType, data },
			warning
		};
	}

	return {
		block: {
			...(currentBlock ?? {}),
			type: nextType
		},
		warning: null
	};
}
