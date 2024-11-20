<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;

	export let code: string;
	export let onDidChangeContent: (
		event: Monaco.editor.IModelContentChangedEvent
	) => void = () => {};

	onMount(async () => {
		monaco = (await import('./monaco')).default;

		const editor = monaco.editor.create(editorContainer, {
			minimap: { enabled: false },
			autoIndent: 'none',
			automaticLayout: true,
			theme: 'vs-dark',
			language: 'python'
		});

		const model = monaco.editor.createModel(code, 'python');
		editor.setModel(model);
		model.setValue(code);

		model.onDidChangeContent((event) => {
			code = model.getValue();
			onDidChangeContent && onDidChangeContent(event);
		});
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});
</script>

<div bind:this={editorContainer}></div>

<style>
	div {
		width: 100%;
		height: 100%;
	}
</style>
