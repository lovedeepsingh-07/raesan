<script lang="ts">
	let { children, class: className = "", ...rest } = $props();
</script>

<button
	{...rest}
	class={`rounded bg-primary px-2 py-1 text-primary-foreground transition-all hover:cursor-pointer hover:bg-primary/80 ${className}`}
	>{@render children()}</button
>
