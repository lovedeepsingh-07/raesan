<script>
	import { Sun, Moon, Menu } from "@lucide/svelte";
	import { toggleMode, mode } from "mode-watcher";
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";
	import { getContext } from "svelte";

	const is_mobile = new IsMobile();
	const sidebar = getContext("sidebar");
</script>

<div
	class={`sticky top-0 right-0 left-0 z-[50] flex items-center ${is_mobile.current ? "justify-between" : "justify-end"} px-4 py-2`}
>
	{#if is_mobile.current}
		<button
			class="rounded px-[6px] py-[4px] hover:cursor-pointer hover:bg-muted"
			onclick={() => {
				sidebar.toggle();
			}}
		>
			<Menu />
		</button>
	{/if}
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
