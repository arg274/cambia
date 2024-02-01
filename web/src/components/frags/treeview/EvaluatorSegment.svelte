<script lang="ts">
    import type { EvaluatorType } from "$lib/types/EvaluatorType";
	import { getScoreVariant } from "$lib/utils";

    export let evaluator: EvaluatorType;
    export let score: string;
    export let combinedScore: string;
</script>

<div class="py-1.5 flex justify-between items-center">
    <span>{evaluator}</span>
    <div class="flex gap-x-1 mr-2">
        <!-- FIXME: Green chips have a contrast issue in light mode -->
        <span class="ml-8 chip {getScoreVariant(score)} rounded-full">{score}</span>
        {#if score !== combinedScore}
            <span class="chip {getScoreVariant(combinedScore)} rounded-full">C{combinedScore}</span>
        {/if}
    </div>
</div>