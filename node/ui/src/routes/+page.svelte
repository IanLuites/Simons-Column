<script lang="ts">
	import Editor from '$lib/Editor.svelte';

	let code: string = 'print("Hello world!")';
</script>

<div>
	<Editor bind:code />
</div>

<style>
	div {
		width: 100%;
		height: 100vh;
	}
</style>
