# Media Upload Page

**Status:** Implemented (`g05.004` proof batch 1)

`MediaUploadPage` is the retained page shell for repeated admin media-upload
routes.

It owns the repeated outer structure:

- page header and back-link
- optional intro copy or warning block
- optional loading state
- upload-level error callout

The route still owns:

- file queue state
- duplicate detection
- upload pipeline orchestration
- replace-file mode logic
- progress and completion behavior

## Usage

```svelte
<script lang="ts">
  import { MediaUploadPage } from "@inflatable-cookie/underlay/templates";

  let replaceMediaId = $state<string | null>(null);
  let error = $state<string | null>(null);
</script>

<MediaUploadPage
  title={replaceMediaId ? "Replace File" : "Upload Media"}
  backHref={replaceMediaId ? `/media/${replaceMediaId}` : "/media"}
  backLabel={replaceMediaId ? "Back to media" : "Back to library"}
  {error}
>
  <div class="upload-container">
    <!-- route-owned queue, replace, progress, and action surfaces -->
  </div>
</MediaUploadPage>
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | No | Page title |
| `subtitle` | `string` | No | Secondary line under the title |
| `backHref` | `string \| null` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `bannerMessage` | `string` | No | Optional page banner message |
| `bannerTone` | `"warning" \| "info" \| "danger"` | No | Banner tone |
| `loading` | `boolean` | No | Show inline loading state |
| `loadingMessage` | `string` | No | Loading copy |
| `error` | `string \| null` | No | Upload-level error message |
| `errorTitle` | `string` | No | Title for the error callout |
| `intro` | `Snippet` | No | Optional block between header and main content |
| `children` | `Snippet` | Yes | Route-owned upload workflow content |

## What It Provides

- `PageHeader`
- optional intro region
- optional inline `PageLoading`
- upload-level error callout
- consistent vertical spacing

## What You Bring

- queue state
- upload/replace actions
- duplicate handling
- progress rendering
- success handling and navigation

## Use It When

- the route is an admin media-upload page
- the outer shell is the normal upload page shape
- the route still owns the upload workflow details

## Do Not Use It When

- the route is not a media-upload workflow
- the page is really a broader media-management console
- you need a fully generic upload shell unrelated to the retained admin media
  system

## See Also

- [Template System Overview](./000-template-system-overview.md)
- [Template API Reference](./template-api-reference.md)
- [EntityTrashPage](./entity-trash-page.md)
