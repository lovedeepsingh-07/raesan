<script>
	import { Button } from "$lib/components/ui/button";
	import { Sun, Moon, Menu } from "@lucide/svelte";
	import { toggleMode, mode } from "mode-watcher";
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar";

	const is_mobile = new IsMobile();
	const sidebar = Sidebar.useSidebar();
</script>

<div
	class={`sticky top-0 right-0 left-0 z-[50] flex items-center ${is_mobile.current ? "justify-between" : "justify-end"} px-4 py-2`}
>
	{#if is_mobile.current}
		<Button
			class="py-[18px] hover:cursor-pointer"
			variant="outline"
			onclick={() => {
				sidebar.toggle();
			}}
		>
			<Menu />
		</Button>
	{/if}
	<Button
		class="py-[18px] hover:cursor-pointer"
		onclick={() => {
			toggleMode();
		}}
	>
		{#if mode.current == "dark"}
			<Sun class="size-5" />
		{:else}
			<Moon class="size-5" />
		{/if}
	</Button>
</div>
