<script lang="ts">
    import type { EvaluatorType } from "$lib/types/EvaluatorType";
    import IconUnknown from '~icons/carbon/unknown';
    import Orpheus from "../../icons/Orpheus.svelte";
	import type { ComponentType } from "svelte";

    interface Props {
        evaluator: EvaluatorType;
    }

    let { evaluator }: Props = $props();

    let icon: ComponentType = $state();

    switch (evaluator) {
        case "OPS":
            icon = Orpheus;
            break;
        default:
            icon = IconUnknown;
            break;
    }

    const SvelteComponent = $derived(icon);
</script>

<div class="w-6">
    <SvelteComponent />
</div>