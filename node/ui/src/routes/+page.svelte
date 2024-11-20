<script lang="ts">
	import Monaco from '$lib/Monaco.svelte';

	let code: string = 'print("Hello world!")';
</script>

<div>
	<Monaco bind:code />
</div>

<style>
	div {
		width: 100%;
		height: 100vh;
	}
</style>
