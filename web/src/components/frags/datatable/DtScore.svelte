<script lang="ts">
	import type { ResponseEntry } from "$lib/types/ResponseEntry";
	import { getScoreVariant, isCambiaResponse } from "$lib/utils";

    export let res: ResponseEntry;
    $: score = res.content && isCambiaResponse(res.content) && res.status === "processed" ? res.content!.evaluation_combined.filter(x => x.evaluator === 'OPS')[0].combined_score : "N/A";
</script>

{#if score}
    <div class="flex chip {getScoreVariant(score)} rounded-full pointer-events-none">{score}</div>
{/if}