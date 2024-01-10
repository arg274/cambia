<script lang="ts">
    import type { Deduction } from "$lib/types/Deduction";

    export let deduction: Deduction;

    // TODO: This only applies to OPS for now
    let neg = typeof deduction.deduction_score === "number" && parseInt(deduction.deduction_score) > 0;
</script>

<div class="grid grid-cols-6 items-center py-0.5">

    <span class="col-span-5 text-xs">{deduction.data.message}</span>
    <span class="col-span-1 w-10 chip py-1 variant-soft-error rounded-full">{neg ? "-" : ""}{deduction.deduction_score}</span>
</div>