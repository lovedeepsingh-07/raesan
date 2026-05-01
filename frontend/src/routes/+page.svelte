<script lang="ts">
	import { CirclePlus, NotebookPen } from "@lucide/svelte";
	import { Button } from "$components";
	import type { RaesanTest } from "$sdk/models";
	import { onMount } from "svelte";
	import { db } from "$lib/database";

	let test_list: Array<RaesanTest> = $state([]);

	onMount(async () => {
		test_list = await db.test_list.toArray();
	});
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
				<p class="text-muted-foreground">You currently have no tests.</p>
			{:else}
				<div
					class="mt-[20px] grid w-full grid-cols-1 gap-[20px] px-4 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
				>
					{#each test_list as curr_test}
						<a
							href={`/tests/${curr_test.id}`}
							class="rounded-lg border bg-card p-2 text-card-foreground transition-all hover:cursor-pointer hover:bg-primary hover:text-primary-foreground"
						>
							{curr_test.id}
						</a>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
