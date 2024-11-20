<script lang="ts">
	import loader from '@monaco-editor/loader';
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;

	export let code: string;

	onMount(async () => {
		const monacoEditor = await import('monaco-editor');
		loader.config({ monaco: monacoEditor.default });
		monaco = await loader.init();

		const editor = monaco.editor.create(editorContainer, {
			minimap: { enabled: false },
			autoIndent: 'none',
			automaticLayout: true,
			theme: 'vs-dark',
			language: 'python'
		});

		const model = monaco.editor.createModel(code, 'python');
		editor.setModel(model);

		model.onDidChangeContent(() => {
			code = model.getValue();
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
