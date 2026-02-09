<script lang="ts" module>
	/**
	 * File state for tracking upload progress.
	 */
	export interface FileUploadItem {
		/** The file object */
		file: File;
		/** Unique ID for this upload */
		id: string;
		/** Upload progress (0-100) */
		progress: number;
		/** Current status */
		status: "pending" | "uploading" | "complete" | "error";
		/** Error message if status is 'error' */
		error?: string;
		/** Preview URL (for images) */
		previewUrl?: string;
		/** Original file (if compressed) */
		originalFile?: File;
	}
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
	import {
		formatFileSize,
		generateFileUploadId,
		validateUploadFile
	} from "./file-upload/helpers";

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
		files.forEach((item) => {
			if (item.previewUrl) {
				URL.revokeObjectURL(item.previewUrl);
			}
		});
	});

	// Process files from input or drop
	async function processFiles(fileList: FileList | null) {
		if (!fileList || fileList.length === 0) return;

		const newFiles: FileUploadItem[] = [];
		const filesToUpload: File[] = [];

		// Check max files limit
		const availableSlots = multiple ? maxFiles - files.length : 1;
		const filesToProcess = Array.from(fileList).slice(0, availableSlots);

		for (const file of filesToProcess) {
			const error = validateUploadFile({
				file,
				maxSize,
				accept,
				validate
			});

			if (error) {
				onError?.({ file, message: error });
				continue;
			}

			// Compress image if enabled
			let processedFile = file;
			let originalFile: File | undefined;

			if (compress && file.type.startsWith("image/")) {
				const compressed = await compressImage(file, compressionOptions);
				if (compressed !== file) {
					originalFile = file;
					processedFile = compressed;
				}
			}

			const item: FileUploadItem = {
				file: processedFile,
				id: generateFileUploadId(),
				progress: 0,
				status: "pending",
				originalFile
			};

			// Generate preview for images
			if (showPreview && processedFile.type.startsWith("image/")) {
				item.previewUrl = URL.createObjectURL(processedFile);
			}

			newFiles.push(item);
			filesToUpload.push(processedFile);
		}

		if (newFiles.length > 0) {
			if (multiple) {
				files = [...files, ...newFiles];
			} else {
				// Clean up old preview URL
				if (files[0]?.previewUrl) {
					URL.revokeObjectURL(files[0].previewUrl);
				}
				files = newFiles;
			}

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
		if (item.previewUrl) {
			URL.revokeObjectURL(item.previewUrl);
		}

		files = files.filter((f) => f.id !== item.id);
		onChange?.(files);
		onRemove?.(item);
	}

	// Retry a failed upload
	function handleRetry(item: FileUploadItem) {
		item.status = "pending";
		item.error = undefined;
		item.progress = 0;
		files = files;
		onUpload?.([item.file]);
	}

	// Update file progress (called externally)
	export function updateProgress(id: string, progress: number) {
		const item = files.find((f) => f.id === id);
		if (item) {
			item.progress = progress;
			item.status = progress < 100 ? "uploading" : "complete";
			files = files;
		}
	}

	// Set file error (called externally)
	export function setError(id: string, message: string) {
		const item = files.find((f) => f.id === id);
		if (item) {
			item.status = "error";
			item.error = message;
			files = files;
		}
	}

	// Clear all files
	export function clear() {
		files.forEach((item) => {
			if (item.previewUrl) {
				URL.revokeObjectURL(item.previewUrl);
			}
		});
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
				<li class="file-item" class:error={item.status === "error"}>
					{#if item.previewUrl}
						<div class="file-preview">
							<img src={item.previewUrl} alt={item.file.name} />
						</div>
					{:else}
						<div class="file-icon">
							<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
								<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
								<polyline points="14 2 14 8 20 8" />
							</svg>
						</div>
					{/if}

					<div class="file-info">
						<span class="file-name">{item.file.name}</span>
						<span class="file-size">{formatFileSize(item.file.size)}</span>
						{#if item.error}
							<span class="file-error">{item.error}</span>
						{/if}
					</div>

					{#if item.status === "uploading"}
						<div class="file-progress">
							<div class="progress-bar">
								<div class="progress-fill" style:width="{item.progress}%"></div>
							</div>
							<span class="progress-text">{item.progress}%</span>
						</div>
					{:else if item.status === "complete"}
						<div class="file-status complete">
							<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<polyline points="20 6 9 17 4 12" />
							</svg>
						</div>
					{:else if item.status === "error"}
						<button type="button" class="retry-button" onclick={() => handleRetry(item)} aria-label="Retry upload">
							<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M23 4v6h-6" />
								<path d="M1 20v-6h6" />
								<path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
							</svg>
						</button>
					{/if}

					<button
						type="button"
						class="remove-button"
						onclick={() => handleRemove(item)}
						aria-label="Remove file"
						disabled={item.status === "uploading"}
					>
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="18" y1="6" x2="6" y2="18" />
							<line x1="6" y1="6" x2="18" y2="18" />
						</svg>
					</button>
				</li>
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

	.file-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--color-surface-subtle, #f8fafc);
		border-radius: var(--radius-md, 0.375rem);
		border: 1px solid var(--color-border, #e2e8f0);
	}

	.file-item.error {
		border-color: var(--color-danger, #ef4444);
		background: var(--color-danger-subtle, #fef2f2);
	}

	.file-preview {
		width: 48px;
		height: 48px;
		border-radius: var(--radius-sm, 0.25rem);
		overflow: hidden;
		flex-shrink: 0;
	}

	.file-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-muted, #64748b);
		flex-shrink: 0;
	}

	.file-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.file-name {
		font-size: 0.875rem;
		font-weight: 500;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-size {
		font-size: 0.75rem;
		color: var(--color-text-muted, #64748b);
	}

	.file-error {
		font-size: 0.75rem;
		color: var(--color-danger, #ef4444);
	}

	.file-progress {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 120px;
	}

	.progress-bar {
		flex: 1;
		height: 4px;
		background: var(--color-border, #e2e8f0);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--color-primary, #3b82f6);
		transition: width 0.2s;
	}

	.progress-text {
		font-size: 0.75rem;
		color: var(--color-text-muted, #64748b);
		width: 35px;
		text-align: right;
	}

	.file-status.complete {
		color: var(--color-success, #22c55e);
	}

	.retry-button,
	.remove-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border: none;
		background: none;
		cursor: pointer;
		border-radius: var(--radius-sm, 0.25rem);
		color: var(--color-text-muted, #64748b);
		transition: background-color 0.2s;
	}

	.retry-button:hover,
	.remove-button:hover {
		background: var(--color-surface-hover, #f1f5f9);
	}

	.remove-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
