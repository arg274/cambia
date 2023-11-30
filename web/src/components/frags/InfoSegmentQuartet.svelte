<script lang="ts">
    import type { Quartet } from "$lib/types/Quartet";
    import IconCheckmarkFilled from '~icons/carbon/checkmark-filled';
    import IconCloseFilled from '~icons/carbon/close-filled';
    import IconHelpFilled from '~icons/carbon/help-filled';
    import IconErrorFilled from '~icons/carbon/error-filled';
    import IconUnknown from '~icons/carbon/unknown';
    
    export let header: string;
    export let value: Quartet;

    export let icon: ConstructorOfATypedSvelteComponent = IconUnknown;
</script>

{#if value}
    <div class="flex flex-col px-2 py-1">
        <div class="flex items-center justify-between">
            <div class="flex items-center">
                <svelte:component this={icon} class="icon-sm" />
                <h4 class="ml-2 dark:font-light text-sm">{header}</h4>
            </div>
            {#if value === 'True'}
                <IconCheckmarkFilled class="text-success-700 visible dark:text-success-400" />
            {:else if value === 'False'}
                <IconCloseFilled class="text-error-700 dark:text-error-400" />
            {:else if value === 'Unknown'}
                <IconHelpFilled class="text-warning-700 dark:text-warning-400" />
            {:else}
                <IconErrorFilled />
            {/if}
        </div>
    </div>
{/if}