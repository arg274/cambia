<script lang="ts">
	import LogView from '../../components/LogView.svelte';

    import { page } from '$app/state'
	import type { CambiaResponse } from '$lib/types/CambiaResponse';
	import type { ResponseEntry } from '$lib/types/ResponseEntry';
	import { hashIndexLookup, responseStore } from '$lib/LogStore';
	import { afterNavigate, goto } from '$app/navigation';
	import { removeRoute } from '$lib/utils';
	import { get } from 'svelte/store';

    let res: CambiaResponse | null = $state(null);

    // TODO: See if this can solved using PageData at some other point
    function lookup(store: ResponseEntry[]): CambiaResponse | null {
        const logId = page.url.searchParams.get("id");
        const indices = logId ? hashIndexLookup.get(logId) : undefined;
        if (indices === undefined || indices.length === 0) return null;
        const entry = store[indices[0]];
        // Outer guards ensure that this never contains a CambiaError
        return entry?.status === 'processed' ? entry.content as CambiaResponse | null : null;
    }

    // Only an id that does not resolve on arrival goes home. Deciding this from
    // the effect instead raced the route a new selection asks for: choosing
    // several logs here reinitialises responseStore, this id stops resolving,
    // and the redirect home beat inputChanged's goto to /logs.
    afterNavigate(() => {
        if (lookup(get(responseStore)) === null) {
            goto(`${removeRoute(location.pathname, page.route.id)}/`);
        }
    });

    $effect(() => {
        res = lookup($responseStore);
    });
</script>

{#if res}
    <!-- Transition bug: https://github.com/sveltejs/svelte/issues/544 -->
    <div class="mt-10 px-4 flex justify-center" id="single-rip-info">
        <div class="w-full xl:w-3/4 2xl:w-1/2 md:max-lg:self-start">
            <LogView res={res} />
        </div>
    </div>
{/if}
