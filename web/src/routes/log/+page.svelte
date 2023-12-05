<script lang="ts">
    import { browser } from '$app/environment';
    import { fade } from 'svelte/transition';
	import LogView from '../../components/LogView.svelte';
    
    import { page } from '$app/stores'
	import { pendingStore, processedStore } from '$lib/LogStore';
	import { hexify } from '$lib/utils';
	import { TableOfContents } from '@skeletonlabs/skeleton';
	import type { CambiaResponse } from '$lib/types/CambiaResponse';
	import { goto } from '$app/navigation';

    let logId: string | null;
    let res: CambiaResponse | undefined;

    $: {
        if (browser) {
            // TODO: See if this can solved using PageData at some other point
            logId = $page.url.searchParams.get("id");
            const filteredLogs = $processedStore.filter(x => hexify(x.id) === logId);
            if (!logId || (filteredLogs.length <= 0 && !$pendingStore.includes(logId))) {
                // FIXME: This seems to be triggering when going back to /logs
                // goto('/');
            } else {
                res = filteredLogs[0];
            }
        }
    }
</script>

{#if res}
    <div class="mt-10 px-4 flex justify-center" id="single-rip-info" transition:fade={{duration: 150}}>
        <div class="w-full xl:w-3/4 2xl:w-1/2 md:max-lg:self-start">
            <LogView res={res} />
        </div>
        <div class="hidden md:flex sticky top-14 self-start">
            <!-- FIXME: Currently scrolls to card header which is usually beneath AppBar -->
            <!-- TODO: XSS vuln consideration: https://github.com/skeletonlabs/skeleton/issues/1987 -->
            <TableOfContents
                class="p-4"
                regionLead="text-spaced-mini"
                regionAnchor="text-spaced-mini" />
        </div>
    </div>
{/if}
