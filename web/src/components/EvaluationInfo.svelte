<script lang="ts">
    import { Accordion, AccordionItem } from "@skeletonlabs/skeleton";
    import IconMeter from '~icons/carbon/meter';
	import Card from "./frags/Card.svelte";
    import type { EvaluationCombined } from "$lib/types/EvaluationCombined";
	import { getScoreVariant } from "$lib/utils";

    export let combinedEvals: EvaluationCombined[];
    export let selectedLogIdx: number = 0;
</script>

<!-- TODO: This will need a massive overhaul to handle colours and goto highlighting -->
<Card header="Evaluations">
    <Accordion regionPanel="space-y-2">
        {#each combinedEvals as combinedEval}
            <AccordionItem>
                <svelte:fragment slot="lead">
                    <IconMeter />
                </svelte:fragment>
                <svelte:fragment slot="summary">
                    <div class="flex justify-between items-center">
                        <span>{combinedEval.evaluator}</span>
                        <div class="flex gap-x-1">
                            <!-- FIXME: Green chips have a contrast issue in light mode -->
                            <span class="ml-8 chip {getScoreVariant(combinedEval.evaluations[selectedLogIdx].score)} rounded-full">{combinedEval.evaluations[selectedLogIdx].score}</span>
                            {#if combinedEval.evaluations[selectedLogIdx].score !== combinedEval.combined_score}
                                <span class="chip {getScoreVariant(combinedEval.combined_score)} rounded-full">C{combinedEval.combined_score}</span>
                            {/if}
                        </div>
                    </div>
                </svelte:fragment>
                <svelte:fragment slot="content">
                    {#each combinedEval.evaluations[selectedLogIdx].deductions as deduction}
                        <div class="grid grid-cols-6 items-center">
                            <span class="col-span-5 text-xs">{deduction.data.message}</span>
                            <span class="col-span-1 w-10 chip py-1 variant-soft-error rounded-full">-{deduction.deduction_score}</span>
                        </div>
                    {/each}
                </svelte:fragment>
            </AccordionItem>
        {/each}
    </Accordion>
</Card>