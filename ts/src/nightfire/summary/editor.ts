import SummaryBookEditor from "./SummaryBookEditor.svelte";
import SummaryCirclesEditor from "./SummaryCirclesEditor.svelte";
import SummaryPieEditor from "./SummaryPieEditor.svelte";
import SummaryDiagramEditor from "./SummaryDiagramEditor.svelte";
import SummarySlideshowEditor from "./SummarySlideshowEditor.svelte";
import SummaryImageSliderEditor from "./SummaryImageSliderEditor.svelte";
import SummaryStepsEditor from "./SummaryStepsEditor.svelte";

import {
  registerSchema,
  registerBlockEditor,
  type SchemaDefinition
} from "../editor-registry";

const SUMMARY_SCHEMA: SchemaDefinition = {
  schema: "acow:content/summary@1",
  mode: "single",
  defaultType: "summary.book"
};

registerSchema(SUMMARY_SCHEMA);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.book",
  "Book (multi-page panels)",
  SummaryBookEditor
);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.circles",
  "Circles (key themes)",
  SummaryCirclesEditor
);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.pie",
  "Pie chart (segments)",
  SummaryPieEditor
);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.diagram",
  "Diagram (image + text panels)",
  SummaryDiagramEditor
);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.slideshow",
  "Slideshow (sequential slides)",
  SummarySlideshowEditor
);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.imageSlider",
  "Image slider (two-image comparison)",
  SummaryImageSliderEditor
);

registerBlockEditor(
  SUMMARY_SCHEMA.schema,
  "summary.steps",
  "Steps (ordered list of steps)",
  SummaryStepsEditor
);
