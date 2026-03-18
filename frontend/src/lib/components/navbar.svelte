<script>
	import { Button } from "$lib/components/ui/button";
	import { Sun } from "@lucide/svelte";
	import { toggleMode } from "mode-watcher";
</script>

<div class="sticky top-0 right-0 left-0 z-[100] flex items-center justify-center px-6 py-2">
	<div
		class="flex w-full max-w-[90%] items-center justify-between rounded-full border-2 bg-background px-4 py-2 shadow-sm"
	>
		<Button href="/" variant="link" class="p-0 text-3xl">raesan</Button>
		<Button
			class="hover:cursor-pointer"
			onclick={() => {
				toggleMode();
			}}><Sun /></Button
		>
	</div>
</div>
