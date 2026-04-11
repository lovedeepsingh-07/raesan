<script>
	import { fade, slide } from "svelte/transition";
</script>

<div
	transition:fade
	class="absolute inset-0 top-0 right-0 bottom-0 left-0 z-[90] flex items-center justify-center bg-black/40"
>
	<div transition:slide class="w-fit rounded-lg border bg-sidebar p-2">
		<p class="text-lg">Initializing Agent...</p>
		<p class="text-sm">This may take upto 60 seconds</p>
	</div>
</div>
