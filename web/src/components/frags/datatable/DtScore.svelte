<script lang="ts">
	import { getScoreVariant } from "$lib/utils";

    export let score: string | null;
</script>

{#if score}
    <div class="flex chip {getScoreVariant(score)} rounded-full grow pointer-events-none">{score}</div>
{/if}