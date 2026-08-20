import MediaEditor from "./MediaEditor.svelte";
import {
  registerBlockEditor,
  registerBlockEmptyChecker
} from "../editor-registry";

// Register the media block editor for schemas that allow media blocks.
// Unregistered schemas fall back to markup via resolveSchemaDefinition,
// so registering for markup covers both.
registerBlockEditor("acow:content/markup", "media", "Media", MediaEditor);

registerBlockEmptyChecker("media", (block) => {
  const mediaId = block?.data?.media_id;
  return !mediaId || typeof mediaId !== "string" || mediaId.trim().length === 0;
});
