<script lang="ts">
    import IconUnknown from '~icons/carbon/unknown';
    import IconCopy from '~icons/carbon/copy';
    import { clipboard } from '@skeletonlabs/skeleton';
	import { copySuccess } from "$lib/utils";
    import { getToastStore } from '@skeletonlabs/skeleton';
	import type { Integrity } from '$lib/types/Integrity';
	import type { ComponentType } from 'svelte';

    const toastStore = getToastStore();

    export let header: string;
    export let hash: string;
    export let icon: ComponentType = IconUnknown;
    export let status: Integrity;

    let bgColor: string;
    switch (status) {
        case "Match":
            bgColor = "success";
            break;
        case "Mismatch":
            bgColor = "error";
            break;
        default:
            bgColor = "surface";
            break;
    }
</script>

{#if hash}
    <div class="flex flex-col">   
        <div class="flex items-center"><svelte:component this={icon} class="icon-sm" /><span class="ml-2 dark:font-light text-sm">{header}</span></div>
        <div class="flex items-center place-items-center justify-between">
            <div class="font-mono grow bg-{bgColor}-400 bg-opacity-25 dark:bg-{bgColor}-900 dark:bg-opacity-25 px-2 py-1 truncate">{hash}</div>
            <div class="flex"><button type="button" class="btn-icon bg-initial hover:variant-soft" use:clipboard={hash} on:click={() => {copySuccess(toastStore)}} ><IconCopy /></button></div>
        </div>
    </div>
{/if}
