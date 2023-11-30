<script lang="ts">
    import IconUnknown from '~icons/carbon/unknown';
    import IconCopy from '~icons/carbon/copy';
    import { clipboard } from '@skeletonlabs/skeleton';
	import { copySuccess } from "$lib/utils";
    import { getToastStore } from '@skeletonlabs/skeleton';

    const toastStore = getToastStore();

    export let header: string;
    export let hash: string;
    export let icon: ConstructorOfATypedSvelteComponent = IconUnknown;
</script>

{#if hash}
    <div class="flex flex-col">   
        <div class="flex items-center"><svelte:component this={icon} class="icon-sm" /><span class="ml-1 dark:font-light text-sm">{header}</span></div>
        <div class="grid grid-cols-10 items-center place-items-center">
            <div class="font-mono justify-self-stretch bg-surface-50-900-token px-2 py-1 col-span-9 whitespace-nowrap overflow-x-scroll hide-scrollbar">{hash}</div>
            <button type="button" class="btn-icon bg-initial hover:variant-soft" use:clipboard={hash} on:click={() => {copySuccess(toastStore)}} ><IconCopy /></button>
        </div>
    </div>
{/if}
