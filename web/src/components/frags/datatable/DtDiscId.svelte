<script lang="ts">
    import IconArrowUpRight from '~icons/carbon/arrow-up-right';
    import MusicBrainz from '../../icons/MusicBrainz.svelte';
	import type { ResponseEntry } from "$lib/types/ResponseEntry";
	import { isCambiaResponse } from '$lib/utils';

    export let res: ResponseEntry;
    $: sources = res.content && isCambiaResponse(res.content) && res.status === "processed" && res.content!.parsed.parsed_logs.reduce((acc, log) => acc + log.toc.raw.entries.length, 0) > 0 ? res.content!.parsed.parsed_logs.map(log => log.toc.mbz) : [];
</script>

<div class="flex flex-col gap-y-1 pr-4">
    {#if sources && sources.length > 0}
        {#each sources as source}
            <a class="chip variant-soft font-mono rounded-full" href={source.url} target="_blank">
                <span class="hidden sm:block text-ellipsis line-clamp-1">{source.hash}</span>
                <div class="sm:hidden w-4">
                    <MusicBrainz />
                </div>
                <IconArrowUpRight class="ml-2 icon-sm" />
            </a>
        {/each}
    {:else}
        <div class="chip variant-soft font-mono rounded-full cursor-default">N/A</div>
    {/if}
</div>