<script>
	import { Sun, Moon } from "@lucide/svelte";
	import { toggleMode, mode } from "mode-watcher";
</script>

<div class="sticky top-0 right-0 left-0 z-[50] flex items-center justify-end px-4 py-2">
	<button
		class="rounded px-[6px] py-[4px] hover:cursor-pointer hover:bg-muted"
		onclick={() => {
			toggleMode();
		}}
	>
		{#if mode.current == "dark"}
			<Sun />
		{:else}
			<Moon />
		{/if}
	</button>
</div>
