<script lang="ts">
	import type { Snippet } from "svelte";
	import { fade, slide } from "svelte/transition";

	let tooltip_state: {
		is_hovering: boolean;
		x: number;
		y: number;
	} = $state({
		is_hovering: false,
		x: 0,
		y: 0
	});

	const mouse_over = (event: MouseEvent) => {
		const el = event.currentTarget as HTMLElement;
		const rect = el.getBoundingClientRect();

		tooltip_state.is_hovering = true;
		tooltip_state.x = rect.left + rect.width / 2;
		tooltip_state.y = rect.top - 8;
	};
	const mouse_leave = () => {
		tooltip_state.is_hovering = false;
	};

	let { children, content }: { children: Snippet; content: Snippet } = $props();
</script>

<div role="tooltip" onmouseover={mouse_over} onfocus={() => {}} onmouseleave={mouse_leave}>
	{@render children()}
</div>

{#if tooltip_state.is_hovering}
	<div
		transition:slide
		style="top: {tooltip_state.y + 2}px; left: {tooltip_state.x}px;"
		class="fixed flex max-w-[280px] translate-x-[-50%] translate-y-[-100%] items-center justify-center rounded-lg border bg-card px-2 py-1 shadow-md"
	>
		{@render content()}
	</div>
{/if}
