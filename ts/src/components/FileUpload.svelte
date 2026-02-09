<script lang="ts" module>
	export type { FileUploadItem } from "./file-upload/types";
	export type { ImageCompressionOptions } from "./file-upload/compression";
	export {
		DEFAULT_COMPRESSION,
		compressImage
	} from "./file-upload/compression";
</script>

<script lang="ts">
	import {
		DEFAULT_COMPRESSION,
		compressImage,
		type ImageCompressionOptions
	} from "./file-upload/compression";
	import type { Snippet } from "svelte";
	import { onDestroy } from "svelte";
	import { formatFileSize } from "./file-upload/helpers";
	import type { FileUploadItem } from "./file-upload/types";
	import {
		processUploadFiles,
		removeUploadItem,
		retryUploadItem,
		revokePreviewUrls,
		setUploadError,
		updateUploadProgress
	} from "./file-upload/state";
	import FileUploadItemRow from "./file-upload/FileUploadItemRow.svelte";

	interface Props {
		/** Accepted file types (e.g., "image/*", ".pdf,.doc") */
		accept?: string;
		/** Maximum file size in bytes */
		maxSize?: number;
		/** Allow multiple files */
		multiple?: boolean;
		/** Maximum number of files (when multiple is true) */
		maxFiles?: number;
		/** Show image previews */
		showPreview?: boolean;
		/** Disable the input */
		disabled?: boolean;
		/** Custom validation function */
		validate?: (file: File) => string | null;
		/** Current files (for controlled component) */
		files?: FileUploadItem[];
		/** Enable image compression before upload */
		compress?: boolean;
		/** Image compression options (when compress is true) */
		compressionOptions?: ImageCompressionOptions;
		/** Callback when files change */
		onChange?: (files: FileUploadItem[]) => void;
		/** Callback when files are ready to upload */
		onUpload?: (files: File[]) => void;
		/** Callback when a validation error occurs */
		onError?: (event: { file: File; message: string }) => void;
		/** Callback when a file is removed */
		onRemove?: (item: FileUploadItem) => void;
		/** Custom dropzone content */
		dropzone?: Snippet;
	}

	let {
		accept = "*",
		maxSize = 10 * 1024 * 1024, // 10MB default
		multiple = false,
		maxFiles = 10,
		showPreview = true,
		disabled = false,
		validate = undefined,
		files = $bindable([]),
		compress = false,
		compressionOptions = DEFAULT_COMPRESSION,
		onChange,
		onUpload,
		onError,
		onRemove,
		dropzone
	}: Props = $props();

	// Internal state
	let inputElement: HTMLInputElement | undefined = $state();
	let isDragging = $state(false);
	let dragCounter = $state(0);

	// Cleanup preview URLs on destroy
	onDestroy(() => {
		revokePreviewUrls(files);
	});

	// Process files from input or drop
	async function processFiles(fileList: FileList | null) {
		const { nextFiles, filesToUpload, replacedPreviewUrls } =
			await processUploadFiles({
				fileList,
				currentFiles: files,
				accept,
				maxSize,
				multiple,
				maxFiles,
				showPreview,
				validate,
				compress,
				compressionOptions,
				onValidationError: onError
			});

		replacedPreviewUrls.forEach((previewUrl) => URL.revokeObjectURL(previewUrl));

		if (filesToUpload.length > 0) {
			files = nextFiles;
			onChange?.(files);
			onUpload?.(filesToUpload);
		}

		// Reset input
		if (inputElement) {
			inputElement.value = "";
		}
	}

	// Handle file input change
	function handleInputChange(event: Event) {
		const input = event.target as HTMLInputElement;
		processFiles(input.files);
	}

	// Handle click to open file picker
	function handleClick() {
		if (!disabled) {
			inputElement?.click();
		}
	}

	// Handle keyboard activation
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" || event.key === " ") {
			event.preventDefault();
			handleClick();
		}
	}

	// Drag and drop handlers
	function handleDragEnter(event: DragEvent) {
		event.preventDefault();
		dragCounter++;
		isDragging = true;
	}

	function handleDragLeave(event: DragEvent) {
		event.preventDefault();
		dragCounter--;
		if (dragCounter === 0) {
			isDragging = false;
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		dragCounter = 0;
		isDragging = false;

		if (!disabled) {
			processFiles(event.dataTransfer?.files ?? null);
		}
	}

	// Remove a file
	function handleRemove(item: FileUploadItem) {
		files = removeUploadItem(files, item);
		onChange?.(files);
		onRemove?.(item);
	}

	// Retry a failed upload
	function handleRetry(item: FileUploadItem) {
		const { nextFiles, retryFile } = retryUploadItem(files, item);
		files = nextFiles;
		onUpload?.([retryFile]);
	}

	// Update file progress (called externally)
	export function updateProgress(id: string, progress: number) {
		files = updateUploadProgress(files, id, progress);
	}

	// Set file error (called externally)
	export function setError(id: string, message: string) {
		files = setUploadError(files, id, message);
	}

	// Clear all files
	export function clear() {
		revokePreviewUrls(files);
		files = [];
		onChange?.(files);
	}
</script>

<div class="underlay-file-upload" class:disabled class:dragging={isDragging}>
	<!-- Drop zone -->
	<div
		class="drop-zone"
		role="button"
		tabindex={disabled ? -1 : 0}
		aria-disabled={disabled}
		onclick={handleClick}
		onkeydown={handleKeydown}
		ondragenter={handleDragEnter}
		ondragleave={handleDragLeave}
		ondragover={handleDragOver}
		ondrop={handleDrop}
	>
		<input
			bind:this={inputElement}
			type="file"
			{accept}
			{multiple}
			{disabled}
			class="visually-hidden"
			onchange={handleInputChange}
			aria-label="File upload"
		/>

		{#if dropzone}
			{@render dropzone()}
		{:else}
			<div class="drop-zone-content">
				<div class="drop-zone-icon">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
						<polyline points="17 8 12 3 7 8" />
						<line x1="12" y1="3" x2="12" y2="15" />
					</svg>
				</div>
				<p class="drop-zone-text">
					{#if isDragging}
						Drop files here
					{:else}
						<span class="drop-zone-link">Click to upload</span> or drag and drop
					{/if}
				</p>
				<p class="drop-zone-hint">
					{#if accept !== "*"}
						{accept.replace(/\./g, "").replace(/,/g, ", ")}
					{/if}
					{#if maxSize}
						(max {formatFileSize(maxSize)})
					{/if}
				</p>
			</div>
		{/if}
	</div>

	<!-- File list -->
	{#if files.length > 0}
		<ul class="file-list" role="list">
			{#each files as item (item.id)}
				<FileUploadItemRow
					{item}
					onRetry={handleRetry}
					onRemove={handleRemove}
				/>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.underlay-file-upload {
		--fu-border: var(--underlay-upload-border, 2px dashed var(--color-border, #e2e8f0));
		--fu-border-active: var(--underlay-upload-border-active, 2px dashed var(--color-primary, #3b82f6));
		--fu-bg: var(--underlay-upload-bg, var(--color-surface, #fff));
		--fu-bg-hover: var(--underlay-upload-bg-hover, var(--color-surface-subtle, #f8fafc));
		--fu-radius: var(--radius-lg, 0.5rem);
	}

	.drop-zone {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 150px;
		padding: 2rem;
		border: var(--fu-border);
		border-radius: var(--fu-radius);
		background: var(--fu-bg);
		cursor: pointer;
		transition:
			border-color 0.2s,
			background-color 0.2s;
	}

	.drop-zone:hover:not(.disabled .drop-zone) {
		background: var(--fu-bg-hover);
	}

	.dragging .drop-zone {
		border: var(--fu-border-active);
		background: var(--fu-bg-hover);
	}

	.disabled .drop-zone {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.visually-hidden {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	.drop-zone-content {
		text-align: center;
	}

	.drop-zone-icon {
		color: var(--color-text-muted, #64748b);
		margin-bottom: 0.5rem;
	}

	.drop-zone-text {
		margin: 0;
		font-size: 0.875rem;
		color: var(--color-text-muted, #64748b);
	}

	.drop-zone-link {
		color: var(--color-primary, #3b82f6);
		font-weight: 500;
	}

	.drop-zone-hint {
		margin: 0.25rem 0 0;
		font-size: 0.75rem;
		color: var(--color-text-muted, #64748b);
	}

	.file-list {
		list-style: none;
		margin: 1rem 0 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

</style>
