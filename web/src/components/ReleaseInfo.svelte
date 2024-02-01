<script lang="ts">
    import MusicBrainz from "./icons/MusicBrainz.svelte";
	import Card from "./frags/Card.svelte";
	import ComboBox from "./frags/ComboBox.svelte";
	import { getCaaCovers, getJoinedArtists, getReleasesFromDiscId } from '$lib/api/MusicBrainzApi';
	import type { ICoverInfo, ILabel, IRelease } from 'musicbrainz-api';
	import { castToType, castToTypeArray } from '$lib/utils';

    export let mbzTocId: string;
    let selectedRelease = "";

    const res = (async () => {
        const _res = await getReleasesFromDiscId(mbzTocId);
        return await _res.json();
    })();

    function castToReleases(releases: any): IRelease[] {
        return castToTypeArray<IRelease>(releases);
    }

    function castToCoverInfo(coverInfo: any): ICoverInfo {
        return castToType<ICoverInfo>(coverInfo);
    }
</script>

{#if mbzTocId}
    <Card header="Release Info" addClass="col-span-2 h-1/4">
        {#await res}
            Loading
        {:then data}
            {#if data.releases.length > 0}
                {@const releases = castToReleases(data.releases)}
                {@const release = selectedRelease ? releases.filter(rel => rel.id === selectedRelease)[0] : releases[0]}
                <div class="flex mt-2 justify-between">
                    <div class="flex items-center">
                        {#await getCaaCovers(release.id)}
                            Loading cover
                        {:then coverRes}
                            {#await coverRes.json()}
                                Parsing JSON
                            {:then coverData} 
                                {@const frontImages = castToCoverInfo(coverData).images}
                                {#if frontImages.length > 0}
                                    <img width="100" height="100" alt="Album by Artist" src={frontImages[0].thumbnails.small} />
                                {/if}
                            {/await}
                        {:catch}
                            Image loading failed
                        {/await}
                        
                        <div class="ml-4 flex flex-col">
                            <span class="text-md">{getJoinedArtists(release['artist-credit'])}</span>
                            <span class="text-2xl font-bold">{release.title}</span>
                        </div>
                    </div>
                    <a type="button" class="btn-icon bg-initial hover:variant-soft self-start" href="https://musicbrainz.org/release/{release.id}" target="_blank">
                        <MusicBrainz />
                    </a>
                </div>
                <div class="mt-4 flex justify-between items-end">
                    <span><ComboBox addTriggerClass="h-6" textClass="text-sm font-mono" items={releases.map(rel => rel.id)} bind:value={selectedRelease} /></span>
                    <span class="text-xs">{release.date}</span>
                </div>
            {/if}
        {:catch}
            Couldn't fetch
        {/await}
    </Card>
{/if}