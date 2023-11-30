<script lang="ts">
    import { browser } from '$app/environment';
    import { fade } from 'svelte/transition';
	import LogView from '../../components/LogView.svelte';
    
    import { page } from '$app/stores'
	import { logStore, pendingStore, processedStore } from '$lib/LogStore';
	import { hexify } from '$lib/utils';
	import { TableOfContents } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';

    let logId: string | null;

    $: {
        if (browser) {
            // TODO: See if this can solved using PageData at some other point
            logId = $page.url.searchParams.get("id");
            const filteredLogs = $processedStore.filter(x => hexify(x.id) === logId);
            if (!logId || (filteredLogs.length <= 0 && !$pendingStore.includes(logId))) {
                // FIXME: This seems to be triggering when going back to /logs
                // goto('/');
            } else {
                logStore.set(filteredLogs[0]);
            }
        }
    }
</script>

<!-- FIXME: Transition does not work when a single log is dropped on LogView -->
<!-- TODO: This is a very odd way to pass props to a component; maybe we don't need a component here at all -->
<div class="mt-10 px-4 flex justify-center" id="single-rip-info" transition:fade={{duration: 150}}>
    <div class="w-full xl:w-3/4 2xl:w-1/2 md:max-lg:self-start">
        <LogView />
    </div>
    <div class="hidden md:flex sticky top-0">
        <!-- TODO: XSS vuln consideration: https://github.com/skeletonlabs/skeleton/issues/1987 -->
        <TableOfContents
            class="mt-4 px-4"
            regionLead="text-spaced-mini"
            regionAnchor="text-spaced-mini" />
    </div>
</div>