<script lang="ts">
	import { Button, SyncingAgentPopup } from "$components";
	import { render_math } from "@raesan/sdk";
	import "katex/dist/katex.min.css";

	let curr_question_index = $state(0);

	let { data } = $props();
</script>

{#await data.chapter_data}
	<SyncingAgentPopup />
{:then curr_chapter}
	{#if curr_chapter.questions.length == 0}
		<p>No questions</p>
	{:else}
		{@const curr_question = curr_chapter.questions[curr_question_index]}
		<div class="items-top flex gap-[6px]">
			<p>{curr_question_index + 1}.</p>
			<div class="flex flex-col items-start gap-[2px]">
				{@html render_math(curr_question.content)}
				<div class="flex flex-col items-start">
					{#each curr_question.options as curr_option}
						<div class="items-top flex gap-[6px]">
							<p>{curr_option.key}.</p>
							{@html render_math(curr_option.value)}
						</div>
					{/each}
				</div>
			</div>
		</div>
		<Button
			onclick={() => {
				curr_question_index += 1;
			}}
			class="flex gap-[5px]">Next</Button
		>
	{/if}
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
