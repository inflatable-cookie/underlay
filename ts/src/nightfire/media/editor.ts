import MediaEditor from "./MediaEditor.svelte";
import {
  registerBlockEditor,
  registerBlockEmptyChecker
} from "../editor-registry";

// Keep the historical Underlay media-library picker available to any schema
// whose Nightfire strategy admits the `media` block type.
registerBlockEditor(null, "media", "Media", MediaEditor);

registerBlockEmptyChecker("media", (block) => {
  const mediaId = block?.data?.media_id;
  return !mediaId || typeof mediaId !== "string" || mediaId.trim().length === 0;
});
