<script lang="ts">
    import type { Quartet } from "$lib/types/Quartet";
    import IconCheckmarkFilled from '~icons/carbon/checkmark-filled';
    import IconCloseFilled from '~icons/carbon/close-filled';
    import IconUnknown from '~icons/carbon/unknown';
	import type { ComponentType } from "svelte";
	import { quartetToVariant } from "$lib/utils";
    
    export let header: string;
    export let value: Quartet;
    export let valueOk: number = 0;

    export let icon: ComponentType = IconUnknown;
</script>

{#if value}
    <div class="flex flex-col px-2 py-1">
        <div class="flex items-center justify-between">
            <div class="flex gap-2 items-center">
                <div class="w-1 h-4 rounded-full {quartetToVariant(value)}"></div>
                <svelte:component this={icon} class="icon-sm" />
                <h4 class="dark:font-light text-sm">{header}</h4>
            </div>
            {#if valueOk && valueOk > 0}
                <IconCloseFilled class="text-error-700 dark:text-error-400" />
            {:else}
                <IconCheckmarkFilled class="text-success-700 visible dark:text-success-400" />
            {/if}
        </div>
    </div>
{/if}