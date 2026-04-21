<script lang="ts">
	import { SyncingAgentPopup, QuestionDisplay } from "$components";
	import { onMount } from "svelte";
	import "katex/dist/katex.min.css";

	let { data } = $props();

	let curr_question_index: number = $state(0);
	onMount(() => {
		const cached_index: number = localStorage.getItem(`${data.chapter_id}_curr_question_index`);
		if (cached_index) {
			curr_question_index = parseInt(cached_index, 10);
		}
	});
</script>

{#await data.chapter_data}
	<SyncingAgentPopup />
{:then curr_chapter}
	{#if curr_chapter.questions.length == 0}
		<p>No questions</p>
	{:else}
		{@const curr_question = curr_chapter.questions[curr_question_index]}
		<QuestionDisplay
			chapter_id={data.chapter_id}
			{curr_question}
			total_questions={curr_chapter.total_questions}
			bind:curr_question_index
		/>
	{/if}
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
