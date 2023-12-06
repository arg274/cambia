<script lang="ts">
    import IconArrowUpRight from '~icons/carbon/arrow-up-right';
    import IconMusicbrainz from '~icons/cambia/musicbrainz';
	import type { ResponseEntry } from "$lib/types/ResponseEntry";

    export let res: ResponseEntry;
    $: sources = res.status === "processed" ? res.content!.parsed.parsed_logs.map(log => log.toc.mbz) : [];
</script>

{#if sources.length > 0}
    <div class="flex flex-col gap-y-1 pr-4">
        {#each sources as source}
            <a class="chip variant-soft font-mono rounded-full" href={source.url} target="_blank"><span class="hidden sm:block">{source.hash}</span><IconMusicbrainz class="sm:hidden icon-sm" /><IconArrowUpRight class="ml-2 icon-sm" /></a>
        {/each}
    </div>
{:else}
    <div class="chip variant-soft font-mono rounded-full">N/A</div>
{/if}