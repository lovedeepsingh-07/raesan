<script lang="ts">
	import { SyncingAgentPopup, QuestionDisplay } from "$components";
	import { onMount } from "svelte";
	import "katex/dist/katex.min.css";
	import type { PageProps } from "./$types";
	import type { Chapter, Question } from "$lib/models";

	let { data }: PageProps = $props();

	let curr_question_index: number = $state(0);
	onMount(() => {
		const cached_index = localStorage.getItem(`${data.chapter_id}_curr_question_index`);
		if (cached_index !== null) {
			curr_question_index = parseInt(cached_index, 10);
		}
	});
</script>

{#await data.chapter_data}
	<SyncingAgentPopup />
{:then curr_chapter: Chapter}
	{#if curr_chapter.questions.length == 0}
		<p>No questions</p>
	{:else}
		{@const curr_question: Question = curr_chapter.questions[curr_question_index]}
		<QuestionDisplay
			storage_id={`chapter_${curr_chapter.id}_curr_question_index`}
			{curr_question}
			total_questions={curr_chapter.total_questions}
			bind:curr_question_index
		/>
	{/if}
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
