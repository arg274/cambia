<script lang="ts">
	import Card from "./frags/Card.svelte";
	import type { Toc } from "$lib/types/Toc";
	import TocSource from "./frags/TocSource.svelte";
    import MusicBrainz from "./icons/MusicBrainz.svelte";
    import Gracenote from "./icons/Gracenote.svelte";
    import IconPlayCircleOutline from "~icons/solar/play-circle-outline";
    import IconAddCircleOutlineRounded from "~icons/material-symbols/add-circle-outline-rounded";

    export let toc: Toc;

</script>

{#if toc.raw.entries.length > 0}
    <Card header="Disc Identifiers">
        <div class="flex flex-col gap-4">
            <TocSource header="MusicBrainz" discid={toc.mbz.hash} url={toc.mbz.url} icon={MusicBrainz} />
            <TocSource header="CUETools" discid={toc.ctdb_tocid.hash} url={toc.ctdb_tocid.url} icon={IconAddCircleOutlineRounded} />
            <TocSource header="Gracenote" discid={toc.gn.hash} icon={Gracenote} />
            <TocSource header="MCDI" discid={toc.mcdi.hash} url={toc.mcdi.url} icon={IconPlayCircleOutline} />
        </div>
    </Card>
{/if}
