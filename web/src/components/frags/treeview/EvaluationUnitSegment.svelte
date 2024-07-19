<script lang="ts">
    import type { EvaluationUnit } from "$lib/types/EvaluationUnit";

    export let evaluation_unit: EvaluationUnit;

    // TODO: This only applies to OPS for now
    let neg = typeof evaluation_unit.unit_score === "number" && parseInt(evaluation_unit.unit_score) > 0;
</script>

<div class="grid grid-cols-6 items-center py-0.5">

    <span class="col-span-5 text-xs">{evaluation_unit.data.message}</span>
    <span class="col-span-1 w-10 chip py-1 variant-soft-error rounded-full">{neg ? "-" : ""}{evaluation_unit.unit_score}</span>
</div>