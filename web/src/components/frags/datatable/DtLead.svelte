<script lang="ts">
	import type { ResponseEntry } from "$lib/types/ResponseEntry";
	import { hexify, nonNullAssert } from "$lib/utils";
	import InfoOverview from "../../InfoOverview.svelte";

    export let res: ResponseEntry;
</script>

<div class="flex flex-col pl-4 gap-4 pb-4">
	{#if res.status === 'queued'}
		<div class="flex flex-col gap-2">
			<span class="font-mono text-xs text-ellipsis line-clamp-1">{res.filename}</span>
			<div class="placeholder" />
		</div>
	{:else if res.status === 'processed'}
		{@const content = nonNullAssert(res.content)}
		<a class="font-mono text-xs text-ellipsis line-clamp-1" href="/log?id={hexify(content.id)}">{res.filename}</a>
		<InfoOverview parsedLogs={content.parsed} evalCombined={content.evaluation_combined.filter(e => e.evaluator === 'Cambia')[0]} />
	{/if}
</div>