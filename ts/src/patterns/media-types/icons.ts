import ImageIcon from "lucide-svelte/icons/image";
import FileTextIcon from "lucide-svelte/icons/file-text";
import VideoIcon from "lucide-svelte/icons/video";
import MusicIcon from "lucide-svelte/icons/music";
import FileIcon from "lucide-svelte/icons/file";
import type { Component } from "svelte";
import { MediaKind, type MediaKind as MediaKindType } from "./enums";

/**
 * Icon component type (lucide-svelte icons)
 */
export type IconComponent = Component<{ size?: number | string; class?: string }>;

/**
 * Get the appropriate icon component for a media kind.
 */
export function getMediaKindIcon(kind: MediaKindType): IconComponent {
  switch (kind) {
    case MediaKind.Image:
      return ImageIcon as unknown as IconComponent;
    case MediaKind.Video:
      return VideoIcon as unknown as IconComponent;
    case MediaKind.Audio:
      return MusicIcon as unknown as IconComponent;
    case MediaKind.Pdf:
      return FileTextIcon as unknown as IconComponent;
    case MediaKind.Document:
      return FileTextIcon as unknown as IconComponent;
    case MediaKind.Other:
    default:
      return FileIcon as unknown as IconComponent;
  }
}
