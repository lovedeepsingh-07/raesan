<script lang="ts">
	let { data } = $props();
</script>

{#await data.chapter_data}
	<p>Loading...</p>
{:then curr_chapter}
	<div class="items-left flex flex-col gap-[20px]">
		<p class="text-3xl">{curr_chapter.title}</p>
		{#each curr_chapter.questions as curr_question, i}
			<div class="items-top flex gap-[6px]">
				<p>{i + 1}.</p>
				<div class="items-left flex flex-col gap-[2px]">
					<p>{@html curr_question.content}</p>
					<div class="items-left flex flex-col">
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
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
