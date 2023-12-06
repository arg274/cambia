<script lang="ts">
    import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import IconHelp from '~icons/carbon/help';
    import classNames from 'classnames';

    import { toCardId, toHeaderId } from '$lib/utils';
    
    export let header: string = "";
    export let addClass: string = "";

    let infoPopup: PopupSettings = {
        event: 'hover',
        target: 'infoPopup',
        placement: 'top-start',
        closeQuery: '',
    }
</script>

<div class={classNames("flex flex-col bg-surface-100-800-token p-4 z-10", addClass)} id={toCardId(header)}>
    {#if header}
        <div class="flex items-center justify-between mb-4">
            <h3 class="text-spaced-mini" id={toHeaderId(header)}>{header}</h3>
            {#if $$slots.tooltip}
                <div class="p-1" use:popup={infoPopup}>
                    <IconHelp class="pointer-events-none" />
                </div>
                <div class="text-sm card bg-surface-100-800-token p-4 whitespace-nowrap shadow-xl" data-popup="infoPopup">
                    <slot name="tooltip"></slot>
                </div>
            {/if}
        </div>
    {/if}
    <slot />
</div>