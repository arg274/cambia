<script lang="ts">
    import IconCopy from '~icons/carbon/copy';
    import IconCheckmark from '~icons/carbon/checkmark';
    import { copyToClipboard } from "$lib/utils";
    import { onDestroy } from 'svelte';

    interface Props {
        /** Text placed on the clipboard. */
        value: string;
        label?: string;
    }

    let { value, label = "Copy to clipboard" }: Props = $props();

    let copied = $state(false);
    let timer: ReturnType<typeof setTimeout> | undefined;

    async function copy() {
        if (!(await copyToClipboard(value))) return;

        copied = true;
        clearTimeout(timer);
        timer = setTimeout(() => (copied = false), 1500);
    }

    onDestroy(() => clearTimeout(timer));

    const fade = "col-start-1 row-start-1 transition-opacity duration-200";
</script>

<button
    type="button"
    class="btn-icon hover:preset-tonal"
    aria-label={copied ? "Copied" : label}
    onclick={copy}
>
    <!-- Both icons share one grid cell so the swap cannot shift layout. -->
    <span class="grid place-items-center">
        <IconCopy class="{fade} {copied ? 'opacity-0' : 'opacity-100'}" />
        <IconCheckmark class="{fade} {copied ? 'opacity-100' : 'opacity-0'}" />
    </span>
</button>
