<script lang="ts">
	import { Button, SyncingAgentPopup } from "$components";
	import { CirclePlus, ChevronUp, ChevronDown } from "@lucide/svelte";
	import { db } from "$lib/database";
	import type { RaesanTest } from "$lib/models";
	import { goto } from "$app/navigation";
	import type { PageProps } from "./$types";
	import type { Exam, Subject, Chapter } from "$lib/models";

	const MAX_QUESTIONS = 50;
	const MIN_QUESTIONS = 10;
	const MAX_CHAPTERS = 10;

	let total_questions: number = $state(10);
	let selected_chapters: Set<string> = $state(new Set());
	let open_sections: Set<string> = $state(new Set());

	let creating_test: boolean = $state(false);

	let { data }: PageProps = $props();
</script>

{#await data.filter_metadata}
	<SyncingAgentPopup />
{:then filter_metadata: Array<Exam>}
	<div class="fixed right-0 bottom-0 z-[40] p-4">
		<Button
			class="flex gap-[5px] bg-primary text-primary-foreground hover:bg-primary/80"
			onclick={async () => {
				creating_test = true;
				const res = await fetch("", {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({
						total_questions,
						selected_chapters: [...selected_chapters]
					})
				});
				const data: { test_data: RaesanTest } = await res.json();
				await db.test_list.add(data.test_data);
				creating_test = false;
				goto("/");
			}}
		>
			{#if creating_test}
				Creating...
			{:else}
				Create <CirclePlus />
			{/if}
		</Button>
	</div>
	<div class="mb-[120px] flex h-full w-full flex-col items-start justify-center gap-[32px]">
		<div class="flex w-full max-w-[260px] flex-col items-start gap-[8px]">
			<p class="text-2xl font-bold">Total Questions</p>
			<div class="flex w-full items-center justify-around gap-[12px]">
				<Button
					onclick={() => {
						if (total_questions == MIN_QUESTIONS || total_questions - 5 < MIN_QUESTIONS) {
							alert(`cannot go less than ${MIN_QUESTIONS}`);
							return;
						}
						total_questions -= 5;
					}}
					class="w-full max-w-[60px] bg-secondary text-secondary-foreground hover:bg-secondary/80"
					>-5</Button
				>
				<Button
					onclick={() => {
						if (total_questions == MIN_QUESTIONS) {
							alert(`cannot go less than ${MIN_QUESTIONS}`);
							return;
						}
						total_questions -= 1;
					}}
					class="w-full max-w-[60px] bg-secondary text-secondary-foreground hover:bg-secondary/80"
					>−</Button
				>
				<span class="text-lg">{total_questions}</span>
				<Button
					onclick={() => {
						if (total_questions == MAX_QUESTIONS) {
							alert(`cannot go more than ${MAX_QUESTIONS}`);
							return;
						}
						total_questions += 1;
					}}
					class="w-full max-w-[60px] bg-secondary text-secondary-foreground hover:bg-secondary/80"
					>+</Button
				>
				<Button
					onclick={() => {
						if (total_questions == MAX_QUESTIONS || total_questions + 5 > MAX_QUESTIONS) {
							alert(`cannot go more than ${MAX_QUESTIONS}`);
							return;
						}
						total_questions += 5;
					}}
					class="w-full max-w-[60px] bg-secondary text-secondary-foreground hover:bg-secondary/80"
					>+5</Button
				>
			</div>
		</div>
		{#each filter_metadata as curr_exam: Exam}
			{#each curr_exam.subjects as curr_subject: Exam}
				{@const is_section_open = open_sections.has(curr_subject.id)}
				<div class="w-full">
					<button
						onclick={() => {
							const set_copy: Set<string> = new Set(open_sections);
							if (is_section_open) {
								set_copy.delete(curr_subject.id);
							} else {
								set_copy.add(curr_subject.id);
							}
							open_sections = set_copy;
						}}
						class="flex items-center justify-between gap-[20px] max-sm:w-full sm:justify-start"
					>
						<p class="text-2xl font-bold">{curr_exam.title} - {curr_subject.title}</p>
						<Button class="rounded-lg hover:bg-muted">
							{#if is_section_open}
								<ChevronUp />
							{:else}
								<ChevronDown />
							{/if}
						</Button>
					</button>
					{#if is_section_open}
						<div
							class="mt-[20px] grid w-full grid-cols-1 gap-[20px] px-4 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
						>
							{#each curr_subject.chapters as curr_chapter: Chapter}
								{@const is_selected = selected_chapters.has(curr_chapter.id)}
								{@const is_disabled = !is_selected && selected_chapters.size >= MAX_CHAPTERS}
								<label
									class={`flex min-h-[60px] w-full flex-col items-start rounded-lg border bg-card p-2 text-card-foreground ${is_disabled ? "bg-muted text-muted-foreground" : "hover:cursor-pointer hover:bg-secondary hover:text-secondary-foreground"}`}
								>
									<input
										type="checkbox"
										checked={is_selected}
										disabled={is_disabled}
										onchange={() => {
											const set_copy: Set<string> = new Set(selected_chapters);
											if (set_copy.has(curr_chapter.id)) {
												set_copy.delete(curr_chapter.id);
											} else {
												if (set_copy.size >= MAX_CHAPTERS) return;
												set_copy.add(curr_chapter.id);
											}
											selected_chapters = set_copy;
										}}
									/>
									<p>{curr_chapter.title}</p>
								</label>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		{/each}
	</div>
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
