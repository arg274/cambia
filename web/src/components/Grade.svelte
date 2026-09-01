<script lang="ts">
	import Card from "./frags/Card.svelte";
    import type { EvaluationCombined } from "$lib/types/EvaluationCombined";

    interface Props {
        evaluations: EvaluationCombined[];
    }

    type Status = 'success' | 'warning' | 'error' | 'surface';

    // Full class names, for the same reason as ChecksumSegment.
    const BORDER: Record<Status, string> = {
        success: 'border-success-700 dark:border-success-500',
        warning: 'border-warning-700 dark:border-warning-500',
        error: 'border-error-700 dark:border-error-500',
        surface: 'border-surface-700 dark:border-surface-500'
    };
    const BLOB_TOP: Record<Status, string> = {
        success: 'bg-success-400',
        warning: 'bg-warning-400',
        error: 'bg-error-400',
        surface: 'bg-surface-400'
    };
    const BLOB_BOTTOM: Record<Status, string> = {
        success: 'bg-success-600',
        warning: 'bg-warning-600',
        error: 'bg-error-600',
        surface: 'bg-surface-600'
    };

    let { evaluations }: Props = $props();

    const opsEvaluations = $derived(evaluations.filter(x => x.evaluator === 'OPS'));

    const grade: { status: Status, statusGrade: string } = $derived.by(() => {
        if (opsEvaluations.length === 0) {
            return { status: "surface", statusGrade: "N/A" };
        }

        const score = parseInt(opsEvaluations[0].combined_score);

        // TODO: Score-based evaluation is dumb; switch to Cambia eval in future
        switch (true) {
            case (score < 0):
                return { status: "error", statusGrade: "F" };
            case (score < 50):
                return { status: "warning", statusGrade: "C" };
            case (score < 80):
                return { status: "warning", statusGrade: "B" };
            case (score < 100):
                return { status: "warning", statusGrade: "A" };
            case (score == 100):
                return { status: "success", statusGrade: "S" };
            default:
                return { status: "surface", statusGrade: "N/A" };
        }
    });

    const statusGrade = $derived(grade.statusGrade);
    const borderClass = $derived(BORDER[grade.status]);
    const blobTopClass = $derived(BLOB_TOP[grade.status]);
    const blobBottomClass = $derived(BLOB_BOTTOM[grade.status]);
</script>
<div class="relative overflow-hidden">
    <Card header="Grade" addClass="relative border-4 {borderClass}">
        <div class="absolute top-4 w-1/4 h-2/3 {blobTopClass} rounded-full filter blur-2xl opacity-50 dark:opacity-30 z-20 mix-blend-darken dark:mix-blend-color-dodge animate-blob"></div>
        <div class="absolute right-4 bottom-4 w-1/4 h-2/3 {blobBottomClass} rounded-full filter blur-2xl opacity-50 dark:opacity-30 z-20 mix-blend-darken dark:mix-blend-color-dodge animate-blob animation-delay-2000"></div>
        <span class="text-6xl font-black self-end">{statusGrade}</span>
    </Card>
</div>
