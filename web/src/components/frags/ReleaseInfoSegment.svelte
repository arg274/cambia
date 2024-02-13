<script lang="ts">
	import { getCaaCovers, getJoinedArtists } from "$lib/api/MusicBrainzApi";
	import { castToType } from "$lib/utils";
	import type { ICoverInfo, IRelease } from "musicbrainz-api";
	import MusicBrainz from "../icons/MusicBrainz.svelte";
	import type { ReleaseInfo } from "$lib/types/ReleaseInfo";
	import classNames from "classnames";

    export let rel: IRelease | null = null;
    export let relLog: ReleaseInfo;
    export let pending: boolean = false;

    function castToCoverInfo(coverInfo: any): ICoverInfo {
        return castToType<ICoverInfo>(coverInfo);
    }

    const placeholder = "placeholder rounded-none";
    const imgDim = "w-[100px] h-[100px]";
    const animate = "animate-pulse";
    const imgError = placeholder;
</script>

<div class="flex flex-col">
    <div class="flex mt-2 justify-start">
        <div class="flex items-center">
            {#if rel}
                {#await getCaaCovers(rel.id)}
                    <div class={classNames(placeholder, imgDim, animate)}></div>
                {:then coverRes}
                    {#await coverRes.json()}
                        <div class={classNames(placeholder, imgDim, animate)}></div>
                    {:then coverData}
                        {@const frontImages = castToCoverInfo(coverData).images}
                        {#if frontImages.length > 0}
                            <img class={imgDim} alt="{relLog.title} by {relLog.artist}" src={frontImages[0].thumbnails.small} />
                        {:else}
                            <div class={classNames(imgError, imgDim)}></div>
                        {/if}
                    {:catch}
                        <div class={classNames(imgError, imgDim)}></div>
                    {/await}
                {:catch}
                    <div class={classNames(imgError, imgDim)}></div>
            {/await}
            {:else}
                <div class={classNames(imgError, imgDim)}></div>
            {/if}
            <div class="ml-4 flex flex-col">
                {#if rel}
                    <span class="text-md">{getJoinedArtists(rel['artist-credit'])}</span>
                    <span class="text-2xl font-bold">{rel.title}</span>
                {:else if pending}
                    <div class="{classNames(placeholder, animate)} h-4 w-8"></div>
                    <div class="{classNames(placeholder, animate)} h-8 w-20 mt-2"></div>
                {:else}
                    <span class="text-md">{relLog.artist}</span>
                    <span class="text-2xl font-bold">{relLog.title}</span>
                {/if}
            </div>
        </div>
    </div>
    <div class="self-end">
        <div class="flex gap-2 items-center">
            <div class="min-w-4">
                <MusicBrainz />
            </div>
            {#if rel}
                <a type="button" class="self-end" href="https://musicbrainz.org/release/{rel.id}" target="_blank">
                        <span class="text-sm font-mono">{rel.id}</span>
                </a>
            {:else if pending}
                <div class="{classNames(placeholder, animate)} h-4 w-72"></div>
            {:else}
                <span class="text-sm font-mono">N/A</span>
            {/if}
        </div>
    </div>
</div>