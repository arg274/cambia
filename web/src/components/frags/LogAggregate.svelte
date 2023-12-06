<script lang="ts">
	import { perfectCount, warningCount, badCount, unknownCount, responseStore, processedCount } from "$lib/LogStore";
	import Card from "./Card.svelte";

    const statusLabelClass = "flex gap-2 items-center rounded-full uppercase tracking-widest text-xs px-2 py-1";
    const statusContainerClass = "flex flex-col gap-1 items-center";
    const statusCountClass = "text-5xl font-black";
</script>

<div class="relative">
    <Card header="{$processedCount}/{$responseStore.length}">
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 mt-4">
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{$perfectCount}</span>
                <div class={`${statusLabelClass} variant-soft-success`}>Perfect</div>
            </div>
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{$warningCount}</span>
                <div class={`${statusLabelClass} variant-soft-warning`}>Warning</div>
            </div>
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{$badCount}</span>
                <div class={`${statusLabelClass} variant-soft-error`}>Bad</div>
            </div>
            <div class={statusContainerClass}>
                <span class={statusCountClass}>{$unknownCount}</span>
                <div class={`${statusLabelClass} variant-soft-surface`}>Unknown</div>
            </div>
        </div>
    </Card>
</div>