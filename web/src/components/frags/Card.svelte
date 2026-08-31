<script lang="ts">
    import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import IconHelp from '~icons/carbon/help';
    import classNames from 'classnames';

    import { toCardId, toHeaderId } from '$lib/utils';
    
    interface Props {
        header?: string;
        addClass?: string;
        tooltip?: import('svelte').Snippet;
        children?: import('svelte').Snippet;
    }

    let {
        header = "",
        addClass = "",
        tooltip,
        children
    }: Props = $props();

    const infoPopup: PopupSettings = $derived({
        event: 'hover',
        target: `${toCardId(header)}-popup`,
        placement: 'top-start',
        closeQuery: '',
    })
</script>

<div class={classNames("flex flex-col bg-surface-100-800-token p-4 z-10", addClass)} id={toCardId(header)}>
    {#if header}
        <div class="flex items-center justify-between mb-4">
            <h3 class="text-spaced-mini" id={toHeaderId(header)}>{header}</h3>
            {#if tooltip}
                <div class="p-1" use:popup={infoPopup}>
                    <IconHelp class="pointer-events-none" />
                </div>
                <div class="text-sm rounded-xl p-4 bg-surface-300/10 backdrop-blur-xl z-max shadow-xl" data-popup="{toCardId(header)}-popup">
                    {@render tooltip?.()}
                </div>
            {/if}
        </div>
    {/if}
    {@render children?.()}
</div>