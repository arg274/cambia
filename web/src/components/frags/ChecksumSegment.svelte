<script lang="ts">
    import IconUnknown from '~icons/carbon/unknown';
    import IconCopy from '~icons/carbon/copy';
	import { copyToClipboard } from "$lib/utils";
	import type { Integrity } from '$lib/types/Integrity';
	import type { IconComponent } from "$lib/types/IconComponent";

    interface Props {
        header: string;
        hash: string;
        icon?: IconComponent;
        status: Integrity;
    }

    let {
        header,
        hash,
        icon = IconUnknown,
        status
    }: Props = $props();

    // Written out in full so Tailwind can see them; interpolated class names
    // are invisible to its scanner.
    const bgClass: string = $derived.by(() => {
        switch (status) {
            case "Match":
                return "bg-success-400/25 dark:bg-success-900/25";
            case "Mismatch":
                return "bg-error-400/25 dark:bg-error-900/25";
            default:
                return "bg-surface-400/25 dark:bg-surface-950/25";
        }
    });
</script>

{#if hash}
    {@const SvelteComponent = icon}
    <div class="flex flex-col">   
        <div class="flex items-center"><SvelteComponent class="icon-sm" /><span class="ml-2 dark:font-light text-sm">{header}</span></div>
        <div class="flex items-center place-items-center justify-between">
            <div class="font-mono grow {bgClass} px-2 py-1 truncate">{hash}</div>
            <div class="flex"><button type="button" class="btn-icon hover:preset-tonal" onclick={() => copyToClipboard(hash)}><IconCopy /></button></div>
        </div>
    </div>
{/if}
