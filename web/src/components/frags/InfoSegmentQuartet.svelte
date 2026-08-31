<script lang="ts">
    import type { Quartet } from "$lib/types/Quartet";
    import IconCheckmarkFilled from '~icons/carbon/checkmark-filled';
    import IconCloseFilled from '~icons/carbon/close-filled';
    import IconUnknown from '~icons/carbon/unknown';
	import type { ComponentType } from "svelte";
	import { quartetToVariant } from "$lib/utils";
    

    interface Props {
        header: string;
        value: Quartet;
        valueOk?: number;
        icon?: ComponentType;
    }

    let {
        header,
        value,
        valueOk = 0,
        icon = IconUnknown
    }: Props = $props();
</script>

{#if value}
    {@const SvelteComponent = icon}
    <div class="flex flex-col px-2 py-1">
        <div class="flex items-center justify-between">
            <div class="flex gap-2 items-center">
                <div class="w-1 h-4 rounded-full {quartetToVariant(value)}"></div>
                <SvelteComponent class="icon-sm" />
                <h4 class="dark:font-light text-sm">{header}</h4>
            </div>
            {#if valueOk && valueOk > 0}
                <IconCloseFilled class="text-error-700 dark:text-error-400 icon-sm" />
            {:else}
                <IconCheckmarkFilled class="text-success-700 visible dark:text-success-400 icon-sm" />
            {/if}
        </div>
    </div>
{/if}