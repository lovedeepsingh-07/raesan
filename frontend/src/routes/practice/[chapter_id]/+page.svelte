<script lang="ts">
	import { SyncingAgentPopup } from "$components";

	let { data } = $props();
</script>

{#await data.chapter_data}
	<SyncingAgentPopup />
{:then curr_chapter}
	<div class="flex flex-col items-start gap-[20px]">
		<p class="text-3xl">{curr_chapter.title}</p>
		<div class="flex flex-col items-start gap-[20px] overflow-x-auto">
			{#each curr_chapter.questions as curr_question, i}
				<div class="items-top flex gap-[6px]">
					<p>{i + 1}.</p>
					<div class="flex flex-col items-start gap-[2px]">
						<p>{@html curr_question.content}</p>
						<div class="flex flex-col items-start">
							{#each curr_question.options as curr_option}
								<div class="items-top flex gap-[6px]">
									<p>{curr_option.key}.</p>
									<p>{@html curr_option.value}</p>
								</div>
							{/each}
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
