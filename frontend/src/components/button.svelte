<script lang="ts">
	let { children, class: className = "", ...rest } = $props();
</script>

<button {...rest} class={`rounded-lg px-2 py-1 transition-all hover:cursor-pointer ${className}`}
	>{@render children()}</button
>
