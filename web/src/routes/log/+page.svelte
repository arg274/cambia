<script lang="ts">
    import { browser } from '$app/environment';
	import LogView from '../../components/LogView.svelte';
    
    import { page } from '$app/stores'
	import type { CambiaResponse } from '$lib/types/CambiaResponse';
	import { hashIndexLookup, responseStore } from '$lib/LogStore';
	import { goto } from '$app/navigation';
	import { removeRoute } from '$lib/utils';

    let logId: string | null;
    let res: CambiaResponse | null;

    $: {
        if (browser) {
            // TODO: See if this can solved using PageData at some other point
            logId = $page.url.searchParams.get("id");
            const indices = logId ? hashIndexLookup.get(logId) : undefined;
            if (indices !== undefined && indices.length > 0 && $responseStore[indices[0]].status === 'processed') {
                // Outer guards ensure that this never contains a CambiaError
                res = $responseStore[indices[0]].content as CambiaResponse | null;
            } else {
                goto(`${removeRoute(location.pathname, $page.route.id)}/`);
            }
        }
    }
</script>

{#if res}
    <!-- Transition bug: https://github.com/sveltejs/svelte/issues/544 -->
    <div class="mt-10 px-4 flex justify-center" id="single-rip-info">
        <div class="w-full xl:w-3/4 2xl:w-1/2 md:max-lg:self-start">
            <LogView res={res} />
        </div>
    </div>
{/if}
