import MarkdownEditor from "./MarkdownEditor.svelte";
import {
  registerSchema,
  registerBlockEditor,
  registerBlockEmptyChecker,
  type SchemaDefinition
} from "../editor-registry";

const markupSchema: SchemaDefinition = {
  schema: "acow:content/markup",
  mode: "multi",
  defaultType: "markdown"
};

registerSchema(markupSchema);

registerBlockEditor(
  markupSchema.schema,
  "markdown",
  "Markdown",
  MarkdownEditor
);

registerBlockEmptyChecker("markdown", (block) => {
  const text = block?.data?.text;
  return !text || typeof text !== "string" || text.trim().length === 0;
});
