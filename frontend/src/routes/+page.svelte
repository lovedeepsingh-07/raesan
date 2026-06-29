<script lang="ts">
	import { CirclePlus, NotebookPen, Trash2 } from "@lucide/svelte";
	import { env } from "$env/dynamic/public";
	import { Button, Tooltip } from "$components";
	import type { RaesanTest, RaesanTest_ChapterSummary } from "$lib/models";
	import { RaesanTestModel } from "$lib/models";
	import { get_date_string } from "$lib";
	import { onMount } from "svelte";
	import { db } from "$lib/database";
	import { z } from "zod";

	let test_list: Array<RaesanTest> = $state([]);

	onMount(async () => {
		test_list = await z.array(RaesanTestModel).parseAsync(await db.test_list.toArray());
	});

	const get_test_display_name = (curr_test: RaesanTest): string => {
		if (curr_test.chapter_summaries.length > 1) {
			return `${curr_test.chapter_summaries[0].chapter_name}, ${curr_test.chapter_summaries[1].chapter_name}...`;
		} else {
			return curr_test.chapter_summaries[0].chapter_name;
		}
	};
	const get_full_test_name = (curr_test: RaesanTest): string => {
		let out = "";
		for (let i = 0; i < curr_test.chapter_summaries.length; i++) {
			out += curr_test.chapter_summaries[i].chapter_name;
			if (i !== curr_test.chapter_summaries.length - 1) {
				out += "|";
			}
		}
		return out;
	};
</script>

{#snippet raesan_test(curr_test: RaesanTest)}
	{@const display_test_name = get_test_display_name(curr_test)}
	{@const full_test_name = get_full_test_name(curr_test)}
	<div
		class="flex items-start justify-center gap-[4px] rounded-lg border bg-card p-2"
		id={curr_test.id}
	>
		<Tooltip>
			{#snippet content()}
				<div class="flex flex-col items-start">
					{#each curr_test.chapter_summaries as chapter_summary: RaesanTest_ChapterSummary, i}
						<p>{i + 1}. {chapter_summary.chapter_name}</p>
					{/each}
				</div>
			{/snippet}
			<a href={`/tests/${curr_test.id}`} class="flex flex-col items-start hover:cursor-pointer">
				<p>{display_test_name}</p>
				<p class="text-sm text-muted-foreground">
					{get_date_string(new Date(curr_test.created_at))}
				</p>
			</a>
		</Tooltip>
		<Button
			class="top-2 right-2 bg-destructive text-destructive-foreground hover:bg-destructive/80"
			onclick={async () => {
				const found_test = test_list.find((test_element) => test_element.id === curr_test.id);
				if (found_test) {
					const should_delete = confirm("Are you absolutely sure ?");
					if (should_delete) {
						await db.test_list.delete(curr_test.id);
						test_list = test_list.filter((item) => item.id !== curr_test.id);

						// also remove the cached question index for the test
						localStorage.removeItem(`raesan_test_${curr_test.id}_curr_question_index`);
					}
				} else {
					alert("something went wrong, there is no such test to delete");
				}
			}}
		>
			<Trash2 class="h-[20px] w-[16px]" />
		</Button>
	</div>
{/snippet}

<div class="flex flex-col gap-[20px]">
	<div class="fond-bold flex items-center gap-[8px] text-5xl max-sm:flex-col max-sm:items-start">
		<p class="">Welcome to</p>
		<p class="font-serif font-normal italic underline decoration-secondary">raesan</p>
	</div>
	<div
		class="w-fit rounded-lg border border-destructive bg-muted p-2 text-sm text-muted-foreground"
	>
		<p>This project is currently under semi-active development.</p>
		<p>
			If you want to give your feedback, go to the
			<a class="text-secondary-foreground underline" href={env.PUBLIC_DISCORD_SERVER_URL}
				>discord server</a
			>.
		</p>
		<p>
			Follow me on <a
				class="text-secondary-foreground underline"
				href="https://codeberg.org/lovedeepsingh07">codeberg</a
			>
			or
			<a class="text-secondary-foreground underline" href="https://github.com/lovedeepsingh-07"
				>github</a
			>
			too if you want.
		</p>
	</div>
	<div class="flex flex-col gap-[20px]">
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
					class="grid w-full grid-cols-1 gap-[20px] px-4 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
				>
					{#each test_list as curr_test: RaesanTest}
						{@render raesan_test(curr_test)}
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
