<script lang="ts">
	import { Button } from "$components";
	import { render_math } from "$lib";
	import type { Question, QuestionOption } from "$lib/models";

	let attempt_data: { attempted: boolean; attempted_option: string | undefined } = $state({
		attempted: false,
		attempted_option: undefined
	});

	$effect(() => {
		curr_question_index;
		attempt_data.attempted = false;
		attempt_data.attempted_option = undefined;
	});

	const attempt_click = (curr_option_key: string) => {
		attempt_data.attempted = true;
		attempt_data.attempted_option = curr_option_key;
	};

	let {
		storage_id,
		curr_question,
		total_questions,
		curr_question_index = $bindable()
	}: {
		storage_id: string;
		curr_question: Question;
		total_questions: number;
		curr_question_index: number;
	} = $props();
</script>

{#snippet control_button(on_click: () => void, button_text: string)}
	<Button
		onclick={() => {
			on_click();
			localStorage.setItem(storage_id, String(curr_question_index));
		}}
		class="flex w-full max-w-[120px] items-center justify-center gap-[5px] bg-accent text-accent-foreground hover:bg-accent/80"
		>{button_text}</Button
	>
{/snippet}

{#snippet question_option(curr_option: QuestionOption, question_answer: string)}
	{@const is_curr_option_attempted =
		attempt_data.attempted && attempt_data.attempted_option == curr_option.key}
	{@const is_curr_option_correct = curr_option.key == question_answer}
	{@const show_correct = attempt_data.attempted && is_curr_option_correct}
	{@const show_error = is_curr_option_attempted && !is_curr_option_correct}
	<Button
		onclick={() => {
			attempt_click(curr_option.key);
		}}
		class={`justify-left flex w-full items-center gap-[14px] rounded-lg border border-accent px-4 py-2 transition-colors hover:cursor-pointer 
			${show_error ? "border-red-400 bg-red-100 text-red-700" : ""}
			${show_correct ? "border-green-400 bg-green-100 text-green-700" : "border-accent"}
		`}
	>
		<div
			class={`flex h-[32px] w-[32px] items-center justify-center rounded-full border border-accent bg-background transition-colors
				${show_error ? "border-red-400 bg-red-200" : ""}
				${show_correct ? "border-green-400 bg-green-200" : "border-accent"}
			`}
		>
			{curr_option.key}
		</div>
		<span
			class={`
			${show_error ? "text-red-700" : ""}
			${show_correct ? "text-green-700" : ""}
		`}
		>
			{@html render_math(curr_option.value)}
		</span>
	</Button>
{/snippet}

<div class="flex w-full items-center justify-center">
	<div>
		<p class="px-2 text-muted-foreground">Question {curr_question_index + 1}/{total_questions}</p>
		<div
			class="flex h-[490px] w-full max-w-[600px] flex-col items-start gap-[25px] rounded-lg border bg-card p-4"
		>
			<div class="h-[200px] w-full overflow-y-auto">
				<p class="text-lg">{@html render_math(curr_question.content)}</p>
			</div>
			<div class="flex w-full flex-col items-start gap-[15px]">
				<div class="flex w-full flex-col items-start gap-[8px]">
					{#each curr_question.options as curr_option: QuestionOption}
						{@render question_option(curr_option, curr_question.answer)}
					{/each}
				</div>
				<div
					class={`flex w-full items-center ${curr_question_index > 0 ? "justify-between" : "justify-end"}`}
				>
					{#if curr_question_index > 0}
						{@render control_button(() => {
							if (curr_question_index > 0) {
								curr_question_index -= 1;
							}
						}, "Previous")}
					{/if}
					{@render control_button(() => {
						if (curr_question_index < total_questions - 1) {
							curr_question_index += 1;
						}
					}, "Next")}
				</div>
			</div>
		</div>
	</div>
</div>
