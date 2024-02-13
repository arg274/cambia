<script lang="ts">
    import Card from "./frags/Card.svelte";
	import { getReleasesFromDiscId } from '$lib/api/MusicBrainzApi';
	import type { IRelease } from 'musicbrainz-api';
	import { castToTypeArray } from '$lib/utils';
	import type { ReleaseInfo } from "$lib/types/ReleaseInfo";
	import ReleaseInfoSegment from "./frags/ReleaseInfoSegment.svelte";

    export let mbzTocId: string;
    export let logRelease: ReleaseInfo;
    let selectedRelease = "";

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
                {@const releases = castToReleases(data.releases)}
                {@const release = selectedRelease ? releases.filter(rel => rel.id === selectedRelease)[0] : releases[0]}
                <ReleaseInfoSegment rel={release} relLog={logRelease} pending={true} />
                
                <!-- TODO: Allow choosing a release if there are multiple -->
                <!-- <div class="mt-4 flex justify-between items-end">
                    <span><ComboBox addTriggerClass="h-6" textClass="text-sm font-mono" items={releases.map(rel => rel.id)} bind:value={selectedRelease} /></span>
                    <span class="text-xs">{release.date}</span>
                </div> -->
            {:else}
                <ReleaseInfoSegment relLog={logRelease} />
            {/if}
        {:catch}
            <ReleaseInfoSegment relLog={logRelease} />
        {/await}
    </Card>
{/if}