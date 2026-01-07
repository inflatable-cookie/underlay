import { registerBlockValidator } from "../validator-registry";
import {
  isPagedSummaryType,
  extractPagedSummaryPages,
  buildPagedSummaryBlock
} from "./paged-summary";

const SUMMARY_SCHEMA = "acow:content/summary@1";

// Validators for paged summary blocks. These ensure that when a summary
// is prepared for persistence, we drop any "empty" pages (no title and
// no text) while keeping meaningful content intact.

const MAX_PIE_SEGMENTS = 5;
const MAX_STEPS = 5;

function pagedSummaryValidator(block: unknown): unknown {
  const anyBlock = block as any;
  if (!isPagedSummaryType(anyBlock?.type)) return block;

  const pages = extractPagedSummaryPages(anyBlock);
  let nonEmpty = pages.filter(
    (page) =>
      (page.title && page.title.trim().length > 0) ||
      page.text.trim().length > 0
  );

  if (anyBlock.type === "summary.pie" && nonEmpty.length > MAX_PIE_SEGMENTS) {
    nonEmpty = nonEmpty.slice(0, MAX_PIE_SEGMENTS);
  }

  if (anyBlock.type === "summary.steps" && nonEmpty.length > MAX_STEPS) {
    nonEmpty = nonEmpty.slice(0, MAX_STEPS);
  }

  return buildPagedSummaryBlock(anyBlock.type, anyBlock, nonEmpty);
}

function imageSummaryValidator(block: unknown): unknown {
  const anyBlock = block as any;
  if (anyBlock?.type !== "summary.diagram" && anyBlock?.type !== "summary.slideshow") {
    return block;
  }

  const pages: any[] = Array.isArray(anyBlock?.data?.pages)
    ? anyBlock.data.pages
    : [];

  const normalisedPages = pages.filter((page) => {
    const title =
      typeof page?.title === "string" ? page.title.trim() : "";
    const body =
      typeof page?.body === "string" ? page.body.trim() : "";
    const imageId =
      typeof page?.image_id === "string" ? page.image_id.trim() : "";

    return title.length > 0 || body.length > 0 || imageId.length > 0;
  });

  return {
    ...anyBlock,
    data: {
      ...(anyBlock.data ?? {}),
      pages: normalisedPages
    }
  };
}

function imageSliderValidator(block: unknown): unknown {
  const anyBlock = block as any;
  if (anyBlock?.type !== "summary.imageSlider") {
    return block;
  }

  const data = (anyBlock.data ?? {}) as any;

  const normalised = {
    subTitle:
      typeof data.subTitle === "string" && data.subTitle.trim().length > 0
        ? data.subTitle
        : null,
    description:
      typeof data.description === "string" &&
        data.description.trim().length > 0
        ? data.description
        : null,
    image1Id:
      typeof data.image1Id === "string" && data.image1Id.trim().length > 0
        ? data.image1Id
        : null,
    image1Alt:
      typeof data.image1Alt === "string" &&
        data.image1Alt.trim().length > 0
        ? data.image1Alt
        : null,
    image2Id:
      typeof data.image2Id === "string" && data.image2Id.trim().length > 0
        ? data.image2Id
        : null,
    image2Alt:
      typeof data.image2Alt === "string" &&
        data.image2Alt.trim().length > 0
        ? data.image2Alt
        : null,
    startPosition:
      data.startPosition === "right" ? "right" : "left"
  };

  return {
    ...anyBlock,
    data: normalised
  };
}

registerBlockValidator(SUMMARY_SCHEMA, "summary.book", pagedSummaryValidator);
registerBlockValidator(SUMMARY_SCHEMA, "summary.circles", pagedSummaryValidator);
registerBlockValidator(SUMMARY_SCHEMA, "summary.pie", pagedSummaryValidator);
registerBlockValidator(SUMMARY_SCHEMA, "summary.steps", pagedSummaryValidator);
registerBlockValidator(SUMMARY_SCHEMA, "summary.diagram", imageSummaryValidator);
registerBlockValidator(SUMMARY_SCHEMA, "summary.slideshow", imageSummaryValidator);
registerBlockValidator(SUMMARY_SCHEMA, "summary.imageSlider", imageSliderValidator);
