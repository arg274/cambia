<script lang="ts">
    import Card from "./frags/Card.svelte";
	import { getReleasesFromDiscId } from '$lib/api/MusicBrainzApi';
	import type { IRelease } from 'musicbrainz-api';
	import { castToTypeArray } from '$lib/utils';
	import type { ReleaseInfo } from "$lib/types/ReleaseInfo";
	import ReleaseInfoSegment from "./frags/ReleaseInfoSegment.svelte";

    export let mbzTocId: string;
    export let logRelease: ReleaseInfo;

    const res = (async () => {
        const _res = await getReleasesFromDiscId(mbzTocId);
        return await _res.json();
    })();

    function castToReleases(releases: any): IRelease[] {
        return castToTypeArray<IRelease>(releases);
    }
</script>

{#if mbzTocId}
    <Card header="Release Info" addClass="col-span-2 h-1/4">
        {#await res}
            <ReleaseInfoSegment relLog={logRelease} pending={true} />
        {:then data}
            {#if !data.error && data.releases.length > 0}
                <ReleaseInfoSegment rels={castToReleases(data.releases)} relLog={logRelease} mbzTocId={mbzTocId} pending={true} />
            {:else}
                <ReleaseInfoSegment relLog={logRelease} />
            {/if}
        {:catch}
            <ReleaseInfoSegment relLog={logRelease} />
        {/await}
    </Card>
{/if}