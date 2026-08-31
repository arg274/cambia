<script lang="ts">
    import IconArrowUpRight from '~icons/carbon/arrow-up-right';
    import IconCopy from '~icons/carbon/copy';
    import IconIbmCloudPakData from '~icons/carbon/ibm-cloud-pak-data';
    import { clipboard } from '@skeletonlabs/skeleton';
	import { copySuccess } from "$lib/utils";
    import { getToastStore } from '@skeletonlabs/skeleton';
	import type { IconComponent } from "$lib/types/IconComponent";

    const toastStore = getToastStore();


    interface Props {
        header: string;
        discid: string;
        url?: string;
        icon?: IconComponent;
    }

    let {
        header,
        discid,
        url = "",
        icon = IconIbmCloudPakData
    }: Props = $props();

    const SvelteComponent = $derived(icon);
</script>
<div class="flex flex-col">
    <div class="flex items-center">
        <div class="min-w-4">
            <SvelteComponent />
        </div>
        <span class="ml-1.5 dark:font-light text-sm">{header}</span>
    </div>
    <div class="flex items-center place-items-center justify-between">
        <div class="font-mono grow bg-success-900 bg-surface-50-900-token px-2 py-1 truncate">{discid}</div>
        <div class="flex">
            <button type="button" class="btn-icon bg-initial hover:variant-soft" use:clipboard={discid} onclick={() => {copySuccess(toastStore)}}><IconCopy /></button>
            <a type="button" class="btn-icon bg-initial hover:variant-soft {url ? "visible" : "invisible"}" href={url} target="_blank"><IconArrowUpRight /></a>
        </div>
    </div>
</div>