<script lang="ts">
    import type { EvaluatorType } from "$lib/types/EvaluatorType";
    import IconUnknown from '~icons/carbon/unknown';
    import Orpheus from "../../icons/Orpheus.svelte";
	import type { ComponentType } from "svelte";

    export let evaluator: EvaluatorType;

    let icon: ComponentType;

    switch (evaluator) {
        case "OPS":
            icon = Orpheus;
            break;
        default:
            icon = IconUnknown;
            break;
    }
</script>

<div class="w-6">
    <svelte:component this={icon} />
</div>