<script lang="ts">
	import { LoadingAgentPopup } from "$components";
	import { setContext } from "svelte";

	let { data, children } = $props();
</script>

{#await data.filter_metadata}
	<LoadingAgentPopup />
{:then filter_metadata}
	{@const _ = setContext("filter_metadata", filter_metadata)}
	{@render children()}
{:catch error}
	<p>error loading filter metadata: {error.message}</p>
{/await}
