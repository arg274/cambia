<script lang="ts">
    import IconCopy from '~icons/carbon/copy';
    import IconCheckmark from '~icons/carbon/checkmark';
    import IconClose from '~icons/carbon/close';
    import { copyToClipboard } from "$lib/utils";
    import { onDestroy } from 'svelte';

    interface Props {
        /** Text placed on the clipboard. */
        value: string;
        label?: string;
    }

    let { value, label = "Copy to clipboard" }: Props = $props();

    let copied = $state(false);
    let failed = $state(false);
    let timer: ReturnType<typeof setTimeout> | undefined;

    async function copy() {
        // No toast any more, so failure has to read on the button too.
        const ok = await copyToClipboard(value);
        copied = ok;
        failed = !ok;
        clearTimeout(timer);
        timer = setTimeout(() => {
            copied = false;
            failed = false;
        }, 1500);
    }

    onDestroy(() => clearTimeout(timer));

    const fade = "col-start-1 row-start-1 transition-opacity duration-200";
</script>

<button
    type="button"
    class="btn-icon hover:preset-tonal"
    aria-label={copied ? "Copied" : failed ? "Copy failed" : label}
    onclick={copy}
>
    <!-- Both icons share one grid cell so the swap cannot shift layout. -->
    <span class="grid place-items-center">
        <IconCopy class="{fade} {copied || failed ? 'opacity-0' : 'opacity-100'}" />
        <IconCheckmark class="{fade} {copied ? 'opacity-100' : 'opacity-0'}" />
        <IconClose class="{fade} text-error-700 dark:text-error-400 {failed ? 'opacity-100' : 'opacity-0'}" />
    </span>
</button>
