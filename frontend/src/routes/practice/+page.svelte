<script lang="ts">
	import { SyncingAgentPopup } from "$components";
	import type { PageProps } from "./$types";
	import type { Exam, Subject, Chapter } from "$lib/models";

	let { data }: PageProps = $props();
</script>

{#await data.filter_metadata}
	<SyncingAgentPopup />
{:then filter_metadata: Array<Exam>}
	<div class="mb-[120px] flex h-full w-full flex-col items-start justify-center gap-[32px]">
		{#each filter_metadata as curr_exam: Exam}
			{#each curr_exam.subjects as curr_subject: Subject}
				<div>
					<p class="text-2xl font-bold">{curr_exam.title} - {curr_subject.title}</p>
					<div
						class="mt-[20px] grid w-full grid-cols-1 gap-[20px] px-4 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
					>
						{#each curr_subject.chapters as curr_chapter}
							<a
								href={`/practice/${curr_chapter.id}`}
								class="border-error flex min-h-[60px] w-full flex-col items-start rounded-lg border bg-card p-2 text-card-foreground transition-all hover:cursor-pointer hover:bg-primary hover:text-primary-foreground"
							>
								<p class="">{curr_chapter.title}</p>
								<p class="">{curr_chapter.total_questions} Questions</p>
							</a>
						{/each}
					</div>
				</div>
			{/each}
		{/each}
	</div>
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
