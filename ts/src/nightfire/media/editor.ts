import MediaEditor from "./MediaEditor.svelte";
import {
  registerBlockEditor,
  registerBlockEmptyChecker
} from "../editor-registry";

// Register the media block editor for schemas that allow media blocks.
// The note@1 schema falls back to markup@1 via resolveSchemaDefinition,
// so registering for markup@1 covers both.
registerBlockEditor("acow:content/markup@1", "media", "Media", MediaEditor);

registerBlockEmptyChecker("media", (block) => {
  const mediaId = block?.data?.mediaId;
  return !mediaId || typeof mediaId !== "string" || mediaId.trim().length === 0;
});
