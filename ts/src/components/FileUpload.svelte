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
	import type { FileUploadItem } from "./file-upload/types";
	import {
		processUploadFiles,
		removeUploadItem,
		retryUploadItem,
		revokePreviewUrls,
		setUploadError,
		updateUploadProgress
	} from "./file-upload/state";
	import FileUploadDropzone from "./file-upload/FileUploadDropzone.svelte";
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

<div
	class="underlay-file-upload"
	class:underlay-disabled={disabled}
	class:underlay-dragging={isDragging}
>
	<FileUploadDropzone
		{accept}
		{multiple}
		{disabled}
		{isDragging}
		{maxSize}
		{dropzone}
		bind:inputElement
		onClick={handleClick}
		onKeydown={handleKeydown}
		onDragEnter={handleDragEnter}
		onDragLeave={handleDragLeave}
		onDragOver={handleDragOver}
		onDrop={handleDrop}
		onInputChange={handleInputChange}
	/>

	<!-- File list -->
	{#if files.length > 0}
		<ul class="underlay-file-list" role="list">
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
		--fu-border: var(
			--underlay-upload-border,
			2px dashed var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35))
		);
		--fu-border-active: var(
			--underlay-upload-border-active,
			2px dashed var(--underlay-color-primary, #3b82f6)
		);
		--fu-bg: var(
			--underlay-upload-bg,
			var(--underlay-color-surface-raised, var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02)))
		);
		--fu-bg-hover: var(
			--underlay-upload-bg-hover,
			var(--underlay-color-surface-hover, rgba(148, 163, 184, 0.12))
		);
		--fu-radius: var(--underlay-upload-radius, var(--underlay-radius-lg, 0.5rem));
	}

	.underlay-file-list {
		list-style: none;
		margin: 1rem 0 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

</style>
