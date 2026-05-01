<script lang="ts">
	import { db } from "$lib/database";
	import type { RaesanTest } from "$sdk/models";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import { render_math } from "$sdk";

	let { data } = $props();

	let curr_test: RaesanTest = $state({});
	onMount(async () => {
		try {
			curr_test = await db.test_list.get(data.test_id);
			console.log(curr_test);
		} catch {
			goto("/");
		}
	});
</script>

<div
	class="mt-[24px] mb-[70px] flex flex-col gap-[15px] overflow-x-auto overflow-y-hidden px-[20px]"
>
	{#each curr_test.questions as question, i}
		<div class="flex gap-[10px]" id={question.id}>
			<p>{i + 1}.</p>
			<div class="flex flex-col">
				<p class="text-lg">{@html render_math(question.content)}</p>
				<div class="flex flex-col gap-[5px]">
					{#each question.options as option}
						<div class="flex gap-[5px]">
							<p>{option.key}:</p>
							<p>{@html render_math(option.value)}</p>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/each}
</div>
