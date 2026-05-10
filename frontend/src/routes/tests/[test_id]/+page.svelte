<script lang="ts">
	import { db } from "$lib/database";
	import type { RaesanTest } from "$lib/models";
	import { RaesanTestModel } from "$lib/models";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import { QuestionDisplay } from "$components";
	import "katex/dist/katex.min.css";
	import type { PageProps } from "./$types";
	import type { Chapter, Question } from "$lib/models";

	let { data } = $props();

	let fetching_test: boolean = $state(true);
	let curr_test: RaesanTest = $state(RaesanTestModel.parse({}));
	let curr_question_index: number = $state(0);

	onMount(async () => {
		try {
			curr_test = RaesanTestModel.parse(await db.test_list.get(data.test_id));
			fetching_test = false;
		} catch {
			goto("/");
		}
		const cached_index = localStorage.getItem(`raesan_test_${curr_test.id}_curr_question_index`);
		if (cached_index !== null) {
			curr_question_index = parseInt(cached_index, 10);
		}
	});
</script>

{#if fetching_test}
	<p>Loading...</p>
{:else}
	{@const curr_question: Question = curr_test.questions[curr_question_index]}
	<QuestionDisplay
		storage_id={`raesan_test_${curr_test.id}_curr_question_index`}
		{curr_question}
		total_questions={curr_test.total_questions}
		bind:curr_question_index
	/>
{/if}
