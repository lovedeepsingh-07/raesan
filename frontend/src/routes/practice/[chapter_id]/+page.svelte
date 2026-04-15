<script lang="ts">
	import { SyncingAgentPopup } from "$components";
	import { onMount } from "svelte";
	import katex from "katex";
	import "katex/dist/katex.min.css";

	const render_math = (input: string): string => {
		return input
			.replace(/\$\$([\s\S]+?)\$\$/g, (_, math) =>
				katex.renderToString(math.trim(), { displayMode: false, throwOnError: false })
			)
			.replace(/\$([^\$]+?)\$/g, (_, math) =>
				katex.renderToString(math.trim(), { displayMode: false, throwOnError: false })
			);
	};

	let { data } = $props();
</script>

{#await data.chapter_data}
	<SyncingAgentPopup />
{:then curr_chapter}
	<div class="mb-[120px] flex flex-col items-start gap-[20px]">
		<p class="text-2xl">{curr_chapter.title}</p>
		<div class="flex w-full flex-col items-start gap-[20px] overflow-x-hidden">
			{#each curr_chapter.questions as curr_question, i}
				<div class="items-top flex gap-[6px]">
					<p>{i + 1}.</p>
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
			{/each}
		</div>
	</div>
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
