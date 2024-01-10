<script lang="ts">
    import type { EvaluatorType } from "$lib/types/EvaluatorType";
    import IconUnknown from '~icons/carbon/unknown';
    import IconOrpheus from '~icons/cambia/orpheus';
	import type { ComponentType } from "svelte";

    export let evaluator: EvaluatorType;

    let icon: ComponentType;

    switch (evaluator) {
        case "OPS":
            icon = IconOrpheus;
            break;
        default:
            icon = IconUnknown;
            break;
    }
</script>

<svelte:component this={icon} class="stroke-black dark:stroke-white stroke-2" />