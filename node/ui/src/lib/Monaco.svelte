<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

	let monaco: typeof Monaco;
	let editor: Monaco.editor.IStandaloneCodeEditor;
	let model: Monaco.editor.ITextModel;
	let container: HTMLElement;

	export let code: string;
	export let onDidChangeContent: (
		event: Monaco.editor.IModelContentChangedEvent
	) => void = () => {};

	onMount(async () => {
		if (!monaco) {
			monaco = (await import('./monaco')).default;
		}

		editor = monaco.editor.create(container, {
			minimap: { enabled: false },
			autoIndent: 'full',
			automaticLayout: true,
			theme: 'vs-dark',
			language: 'python'
		});

		model = monaco.editor.createModel(code, 'python');
		editor.setModel(model);
		model.setValue(code);

		model.onDidChangeContent((event) => {
			code = model.getValue();
			onDidChangeContent && onDidChangeContent(event);
		});
	});

	$: {
		let _ = code;
		model && model.getValue() != code && model.setValue(code);
	}

	onDestroy(() => {
		model?.dispose();
		editor?.dispose();
	});

	export function dispose() {
		editor?.dispose();
	}
</script>

<div bind:this={container}></div>

<style>
	div {
		width: 100%;
		height: 100%;
	}
</style>
