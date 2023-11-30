<script lang="ts">
	import { fileMap } from "$lib/LogStore";
	import type { CambiaResponse } from "$lib/types/CambiaResponse";
	import { hexify, nonNullAssert } from "$lib/utils";
	import InfoOverview from "../InfoOverview.svelte";

    export let res: CambiaResponse;
</script>

<div class="flex flex-col pl-4 gap-4 pb-4">
    <a class="font-mono text-xs grow whitespace-nowrap overflow-x-scroll hide-scrollbar" href="/log?id={hexify(nonNullAssert(res.id))}">{fileMap.get(hexify(nonNullAssert(res.id)))}</a>
    <InfoOverview parsedLogs={res.parsed} evalCombined={res.evaluation_combined.filter(e => e.evaluator === 'Cambia')[0]} />
</div>