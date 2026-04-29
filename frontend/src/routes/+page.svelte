<script lang="ts">
	import { CirclePlus, NotebookPen } from "@lucide/svelte";
	import { Button } from "$components";
	let test_list = $state([]);
</script>

<div class="flex flex-col gap-[20px]">
	<div><p class="text-4xl italic">Welcome to raesan</p></div>
	<div class="flex flex-col gap-[10px]">
		<div class="justify-left flex items-start max-xs:flex-col xs:gap-[10px]">
			<p class="text-2xl">Your Tests</p>
			<div class="flex items-center gap-[10px]">
				<a href="/tests">
					<Button class="flex gap-[5px] bg-primary text-primary-foreground hover:bg-primary/80"
						>Create <CirclePlus /></Button
					>
				</a>
				<a href="/practice">
					<Button class="flex gap-[5px] bg-primary text-primary-foreground hover:bg-primary/80">
						Practice <NotebookPen />
					</Button>
				</a>
			</div>
		</div>
		<div>
			{#if test_list.length == 0}
				<p class="text-muted-foreground">You currently have no test.</p>
			{:else}
				<div class="flex items-center justify-between gap-[10xp]">
					{#each test_list as curr_test}
						<div class="bg-card p-2"></div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
