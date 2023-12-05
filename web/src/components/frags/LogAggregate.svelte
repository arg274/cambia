<script lang="ts">
	import { fileListStore, fileMap, pendingStore, processedStore } from "$lib/LogStore";
	import Card from "./Card.svelte";

    let perfectCount = 0;
    let warningCount = 0;
    let badCount = 0;
    let unknownCount = 0;

    const statusLabelClass = "flex gap-2 items-center rounded-full uppercase tracking-widest text-xs px-2 py-1";
    const statusContainerClass = "flex flex-col gap-1 items-center";
    const statusCountClass = "text-5xl font-black";

    processedStore.subscribe(logs => {
        if (logs.length <= 0) {
            perfectCount = 0;
            warningCount = 0;
            badCount = 0;
            unknownCount = 0;
            return;
        }
        const newRes = logs[logs.length - 1];
        const opsEval = newRes.evaluation_combined.filter(x => x.evaluator === 'OPS');
        if (opsEval.length > 0) {
            // TODO: Score-based evaluation is dumb; switch to Cambia eval in future
            const score = parseInt(opsEval[0].combined_score);
            switch (true) {
                case (score < 0):
                    badCount++;
                    break;
                case (score < 100):
                    warningCount++;
                    break;
                case (score == 100):
                    perfectCount++;
                    break;
                default:
                    break;
            }
        }
        // TODO: This shouldn't update with each processed log
        // unknownCount = $fileListStore && $pendingStore && $pendingStore.length == 0 ? ($fileListStore.length - logs.length) : 0;
    });
</script>

<div class="relative">
    <Card header="Status">
        <div class="grid grid-cols-4 gap-2 mt-4">
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{perfectCount}</span>
                <div class={`${statusLabelClass} variant-soft-success`}>Perfect</div>
            </div>
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{warningCount}</span>
                <div class={`${statusLabelClass} variant-soft-warning`}>Warning</div>
            </div>
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{badCount}</span>
                <div class={`${statusLabelClass} variant-soft-error`}>Bad</div>
            </div>
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{unknownCount}</span>
                <div class={`${statusLabelClass} variant-soft-surface`}>Unknown</div>
            </div>
        </div>
    </Card>
</div>