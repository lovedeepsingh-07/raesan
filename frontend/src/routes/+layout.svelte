<script lang="ts">
	import "./layout.css";
	import { Navbar } from "$components";
	import { ModeWatcher } from "mode-watcher";

	let { children } = $props();
</script>

<svelte:head>
	<title>raesan</title>
	<link rel="icon" type="image/png" href="/icon.png" />
</svelte:head>

<ModeWatcher defaultMode={"dark"} />

<Navbar />
<div class="mx-auto w-full max-w-[78%]">
	{@render children()}
</div>
