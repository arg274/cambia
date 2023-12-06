<script lang="ts">
	import Card from "./frags/Card.svelte";
    import type { EvaluationCombined } from "$lib/types/EvaluationCombined";

    export let evaluations: EvaluationCombined[];

    const opsEvaluations = evaluations.filter(x => x.evaluator === 'OPS');

    let status: string = "surface";
    let statusGrade: string = "N/A";

    if (opsEvaluations.length > 0) {
        const score = parseInt(opsEvaluations[0].combined_score);
        
        // TODO: Score-based evaluation is dumb; switch to Cambia eval in future
        switch (true) {
            case (score < 0):
                status = "error";
                statusGrade = "F"
                break;
            case (score < 50):
                status = "warning";
                statusGrade = "C";
                break;
            case (score < 80):
                status = "warning";
                statusGrade = "B";
                break;
            case (score < 100):
                status = "warning";
                statusGrade = "A"
                break;
            case (score == 100):
                status = "success";
                statusGrade = "S"
            default:
                break;
        }
    }
</script>
<div class="relative">
    <!-- FIXME: This seems like some CSS pruning bug; I shouldn't have to explicitly mention these in the class(es) -->
    <div class="hidden border-success-700 dark:border-success-500"></div>
    <div class="hidden border-error-700 dark:border-error-500"></div>
    <div class="hidden border-warning-700 dark:border-warning-500"></div>
    <div class="hidden bg-success-400 dark:border-success-500"></div>
    <div class="hidden bg-error-400 dark:border-error-500"></div>
    <div class="hidden bg-warning-400 dark:border-warning-500"></div>
    <div class="hidden bg-success-600 dark:border-success-500"></div>
    <div class="hidden bg-error-600 dark:border-error-500"></div>
    <div class="hidden bg-warning-600 dark:border-warning-500"></div>
    <Card header="Grade" addClass={`relative border-4 !border-${status}-700 dark:border-${status}-500`}>
        <div class={`absolute top-4 w-1/4 h-2/3 bg-${status}-400 rounded-full filter blur-2xl opacity-50 dark:opacity-30 z-20 mix-blend-darken dark:mix-blend-color-dodge animate-blob`}></div>
        <div class={`absolute right-4 bottom-4 w-1/4 h-2/3 bg-${status}-600 rounded-full filter blur-2xl opacity-50 dark:opacity-30 z-20 mix-blend-darken dark:mix-blend-color-dodge animate-blob animation-delay-2000`}></div>
        <span class="text-6xl font-black self-end">{statusGrade}</span>
    </Card>
</div>