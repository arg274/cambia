<script lang="ts">
    import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import IconHelp from '~icons/carbon/help';
    import classNames from 'classnames';

    import { toCardId, toHeaderId } from '$lib/utils';
    
    export let header: string = "";
    export let addClass: string = "";

    let infoPopup: PopupSettings = {
        event: 'hover',
        target: `${toCardId(header)}-popup`,
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
                <div class="text-sm rounded-xl p-4 bg-surface-300/10 backdrop-blur-xl z-max shadow-xl" data-popup="{toCardId(header)}-popup">
                    <slot name="tooltip"></slot>
                </div>
            {/if}
        </div>
    {/if}
    <slot />
</div>