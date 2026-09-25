<script lang="ts">
    import { Tooltip } from 'bits-ui';
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
</script>

<div class={classNames("flex flex-col bg-surface-100-900 p-4 z-10", addClass)} id={toCardId(header)}>
    {#if header}
        <div class="flex items-center justify-between mb-4">
            <h3 class="text-spaced-mini" id={toHeaderId(header)}>{header}</h3>
            {#if tooltip}
                <Tooltip.Provider>
                    <Tooltip.Root delayDuration={100} disableCloseOnTriggerClick>
                        <Tooltip.Trigger class="p-1">
                            <IconHelp class="pointer-events-none" />
                        </Tooltip.Trigger>
                        <Tooltip.Content
                            side="top"
                            align="start"
                            sideOffset={4}
                            class="text-sm rounded-xl p-4 bg-surface-300/10 backdrop-blur-xl z-max shadow-xl"
                        >
                            {@render tooltip?.()}
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider>
            {/if}
        </div>
    {/if}
    {@render children?.()}
</div>
