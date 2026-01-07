export type PagedSummaryType =
  | "summary.book"
  | "summary.circles"
  | "summary.pie"
  | "summary.steps";

export type SummaryPageSnapshot = {
  title: string | null;
  text: string;
};

export function isPagedSummaryType(
  type: string | undefined
): type is PagedSummaryType {
  return (
    type === "summary.book" ||
    type === "summary.circles" ||
    type === "summary.pie" ||
    type === "summary.steps"
  );
}

function extractTextFromBody(body: unknown): string {
  if (typeof body !== "string") return "";
  return body;
}

export function extractPagedSummaryPages(block: unknown): SummaryPageSnapshot[] {
  const anyBlock = block as any;
  const pages: any[] = Array.isArray(anyBlock?.data?.pages)
    ? anyBlock.data.pages
    : [];

  return pages.map((page) => ({
    title:
      typeof page?.title === "string" && page.title.length > 0
        ? page.title
        : null,
    text: extractTextFromBody(page?.body as string | null)
  }));
}

function buildPageBodies(
  pages: SummaryPageSnapshot[]
): { title: string | null; body: string }[] {
  return pages.map((page) => {
    const text = page.text ?? "";

    return {
      title: page.title,
      body: text
    };
  });
}

export function buildPagedSummaryBlock(
  targetType: PagedSummaryType,
  current: unknown,
  pages: SummaryPageSnapshot[]
): unknown {
  const anyCurrent = current as any;
  const version: string =
    typeof anyCurrent?.version === "string"
      ? anyCurrent.version
      : "initial";

  const pageBodies = buildPageBodies(pages);

  const currentData = (anyCurrent?.data ?? {}) as any;

  if (targetType === "summary.book") {
    // Book has no subtitle or other metadata; just pages.
    return {
      type: "summary.book",
      version,
      hash: "",
      data: {
        pages: pageBodies
      }
    };
  }

  if (targetType === "summary.circles") {
    // Circles supports a subtitle + pages.
    return {
      type: "summary.circles",
      version,
      hash: "",
      data: {
        subTitle:
          typeof currentData.subTitle === "string"
            ? currentData.subTitle
            : null,
        pages: pageBodies
      }
    };
  }

  if (targetType === "summary.pie") {
    // Pie supports a subtitle + pages.
    return {
      type: "summary.pie",
      version,
      hash: "",
      data: {
        subTitle:
          typeof currentData.subTitle === "string"
            ? currentData.subTitle
            : null,
        pages: pageBodies
      }
    };
  }

  if (targetType === "summary.steps") {
    // Steps supports a subtitle + pages.
    return {
      type: "summary.steps",
      version,
      hash: "",
      data: {
        subTitle:
          typeof currentData.subTitle === "string"
            ? currentData.subTitle
            : null,
        pages: pageBodies
      }
    };
  }

  return current;
}
