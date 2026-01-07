import SummaryPanelRenderer from "./SummaryPanelRenderer.svelte";
import SummaryBookRenderer from "./SummaryBookRenderer.svelte";
import SummaryCirclesRenderer from "./SummaryCirclesRenderer.svelte";
import SummaryDiagramRenderer from "./SummaryDiagramRenderer.svelte";
import SummarySlideshowRenderer from "./SummarySlideshowRenderer.svelte";
import SummaryImageSliderRenderer from "./SummaryImageSliderRenderer.svelte";
import SummaryStepsRenderer from "./SummaryStepsRenderer.svelte";

import { registerBlockRenderer } from "../render-registry";

const SUMMARY_SCHEMA = "acow:content/summary@1";

// Block renderers for summary‑oriented layouts and panels.

registerBlockRenderer(null, "summary.panel", SummaryPanelRenderer);

registerBlockRenderer(SUMMARY_SCHEMA, "summary.book", SummaryBookRenderer);
registerBlockRenderer(SUMMARY_SCHEMA, "summary.circles", SummaryCirclesRenderer);
registerBlockRenderer(SUMMARY_SCHEMA, "summary.diagram", SummaryDiagramRenderer);
registerBlockRenderer(
  SUMMARY_SCHEMA,
  "summary.slideshow",
  SummarySlideshowRenderer
);
registerBlockRenderer(
  SUMMARY_SCHEMA,
  "summary.imageSlider",
  SummaryImageSliderRenderer
);
registerBlockRenderer(SUMMARY_SCHEMA, "summary.steps", SummaryStepsRenderer);
