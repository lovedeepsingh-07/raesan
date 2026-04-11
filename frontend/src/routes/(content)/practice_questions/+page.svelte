<script lang="ts">
	import { getContext } from "svelte";

	const filter_metadata = getContext("filter_metadata");
</script>

<div
	class="items-left mb-[120px] flex h-full w-full flex-col justify-center rounded-lg border bg-card p-3"
>
	{#each filter_metadata as curr_exam}
		<p class="text-4xl">{curr_exam.title}</p>
		<div>
			{#each curr_exam.subjects as curr_subject}
				<p class="text-2xl font-bold">{curr_subject.title}</p>
				<div>
					{#each curr_subject.chapters as curr_chapter}
						<p>{curr_chapter.title}</p>
					{/each}
				</div>
			{/each}
		</div>
	{/each}
</div>
