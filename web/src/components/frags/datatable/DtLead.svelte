<script lang="ts">
	import type { ResponseEntry } from "$lib/types/ResponseEntry";
	import { hexify, isCambiaError, isCambiaResponse, nonNullAssert } from "$lib/utils";
	import InfoOverview from "../../InfoOverview.svelte";

    export let res: ResponseEntry;
</script>

<div class="flex flex-col pl-4 gap-4 pb-4">
	{#if res.status === 'queued'}
		<div class="flex flex-col gap-2">
			<span class="font-mono text-xs text-ellipsis line-clamp-1">{res.filename}</span>
			<div class="placeholder" />
		</div>
	{:else if res.content && isCambiaResponse(res.content) && res.status === "processed"}
		{@const content = nonNullAssert(res.content)}
		<a class="font-mono text-xs text-ellipsis line-clamp-1" href="/log?id={hexify(content.id)}">{res.filename}</a>
		<InfoOverview parsedLogs={content.parsed} evalCombined={content.evaluation_combined.filter(e => e.evaluator === 'OPS')[0]} />
	{:else if res.content && isCambiaError(res.content) && res.status === "errored"}
		<div class="flex flex-col gap-2">
			<span class="font-mono text-xs text-ellipsis line-clamp-1">{res.filename}</span>
			<span class="text-sm text-error-400">{res.content.message}</span>
		</div>
	{/if}
</div>