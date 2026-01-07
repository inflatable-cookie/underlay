import MarkdownEditor from "./MarkdownEditor.svelte";
import {
  registerSchema,
  registerBlockEditor,
  type SchemaDefinition
} from "../editor-registry";

const markupSchema: SchemaDefinition = {
  schema: "acow:content/markup@1",
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
