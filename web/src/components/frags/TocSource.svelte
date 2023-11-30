<script lang="ts">
    import IconArrowUpRight from '~icons/carbon/arrow-up-right';
    import IconCopy from '~icons/carbon/copy';
    import IconIbmCloudPakData from '~icons/carbon/ibm-cloud-pak-data';
    import { clipboard } from '@skeletonlabs/skeleton';
	import { copySuccess } from "$lib/utils";
    import { getToastStore } from '@skeletonlabs/skeleton';

    const toastStore = getToastStore();

    export let header: string;
    export let discid: string;
    export let url: string = "";

    export let icon: ConstructorOfATypedSvelteComponent = IconIbmCloudPakData;
</script>
<div class="flex flex-col">
    <div class="flex items-center"><svelte:component this={icon} class="icon-sm" /><span class="ml-1 dark:font-light text-sm">{header}</span></div>
    <div class="grid grid-cols-10 items-center place-items-center">
        <div class="font-mono justify-self-stretch bg-surface-50-900-token px-2 py-1 col-span-8 whitespace-nowrap overflow-x-scroll hide-scrollbar">{discid}</div>
        <button type="button" class="btn-icon bg-initial hover:variant-soft" use:clipboard={discid} on:click={() => {copySuccess(toastStore)}}><IconCopy /></button>
        {#if url}
            <a type="button" class="btn-icon bg-initial hover:variant-soft" href={url} target="_blank"><IconArrowUpRight /></a>
        {/if}
    </div>
</div>