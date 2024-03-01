<script lang="ts">
	import type { ComponentType } from 'svelte';
    import IconUnknown from '~icons/carbon/unknown';
    import IconCheckmarkFilled from '~icons/carbon/checkmark-filled';
    import IconCloseFilled from '~icons/carbon/close-filled';
    
    export let header: string = '';
    export let value: string | number | boolean | null | undefined;

    export let icon: ComponentType = IconUnknown;
    export let valueOk: number | null | undefined = null;
</script>

{#if value && !value.toString().toLocaleLowerCase().startsWith("null") && !value.toString().toLocaleLowerCase().startsWith("undefined")}
    <div class="flex flex-col gap-0.5">
        <div class="flex items-center">
            <svelte:component this={icon} class="icon-sm" /> <h4 class="ml-1.5 dark:font-light text-sm">{header}</h4>
        </div>
        <div class="flex items-center gap-2">
            <div class="flex gap-2 items-center">
                <span class="text-xl font-bold">{value}</span>
                <slot name="extra"></slot>
            </div>
            {#if valueOk && valueOk > 0}
                <IconCloseFilled class="text-error-700 dark:text-error-400" />
            {:else if valueOk == 0}
                <IconCheckmarkFilled class="text-success-700 visible dark:text-success-400" />
            {/if}
        </div>
    </div>
{/if}