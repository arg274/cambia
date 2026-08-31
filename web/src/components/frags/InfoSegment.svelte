<script lang="ts">
	import type { IconComponent } from "$lib/types/IconComponent";
    import IconUnknown from '~icons/carbon/unknown';
    import IconCheckmarkFilled from '~icons/carbon/checkmark-filled';
    import IconCloseFilled from '~icons/carbon/close-filled';
    

    interface Props {
        header?: string;
        value: string | number | boolean | null | undefined;
        icon?: IconComponent;
        valueOk?: number | null | undefined;
        extra?: import('svelte').Snippet;
    }

    let {
        header = '',
        value,
        icon = IconUnknown,
        valueOk = null,
        extra
    }: Props = $props();
</script>

{#if value && !value.toString().toLocaleLowerCase().startsWith("null") && !value.toString().toLocaleLowerCase().startsWith("undefined")}
    {@const SvelteComponent = icon}
    <div class="flex flex-col gap-0.5">
        <div class="flex items-center">
            <SvelteComponent class="icon-sm" /> <h4 class="ml-1.5 dark:font-light text-sm">{header}</h4>
        </div>
        <div class="flex items-center gap-2">
            <div class="flex gap-2 items-center">
                <span class="text-xl font-bold">{value}</span>
                {@render extra?.()}
            </div>
            {#if valueOk && valueOk > 0}
                <IconCloseFilled class="text-error-700 dark:text-error-400 icon-sm" />
            {:else if valueOk == 0}
                <IconCheckmarkFilled class="text-success-700 visible dark:text-success-400 icon-sm" />
            {/if}
        </div>
    </div>
{/if}