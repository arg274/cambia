<script lang="ts">
	import Card from "./frags/Card.svelte";
	import type { Toc } from "$lib/types/Toc";
	import TocSource from "./frags/TocSource.svelte";
    import IconGracenote from '~icons/cambia/gracenote';
    import IconMusicbrainz from '~icons/cambia/musicbrainz';

    export let toc: Toc;

</script>

{#if toc.raw.entries.length > 0}
    <Card header="Disc Identifiers">
        <div class="flex flex-col gap-4">
            <TocSource header="MusicBrainz" discid={toc.mbz.hash} url={toc.mbz.url} icon={IconMusicbrainz} />
            <TocSource header="CUETools" discid={toc.ctdb_tocid.hash} url={toc.ctdb_tocid.url} />
            <TocSource header="Gracenote" discid={toc.gn.hash} icon={IconGracenote} />
            <TocSource header="MCDI" discid={toc.mcdi.hash} url={toc.mcdi.url} />
        </div>
    </Card>
{/if}
