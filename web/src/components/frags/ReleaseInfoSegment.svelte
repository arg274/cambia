<script lang="ts">
	import { getCaaCovers, getJoinedArtists } from "$lib/api/MusicBrainzApi";
	import { castToType } from "$lib/utils";
	import type { ICoverInfo, IImage, IRelease } from "musicbrainz-api";
	import MusicBrainz from "../icons/MusicBrainz.svelte";
	import type { ReleaseInfo } from "$lib/types/ReleaseInfo";
	import classNames from "classnames";
	import { onMount } from "svelte";

    export let rels: IRelease[] = [];
    export let relLog: ReleaseInfo;
    export let pending: boolean = false;
    export let mbzTocId: string = "";

    let frontImages: IImage[] = [];
    let errored: boolean = false;

    const placeholder = "placeholder rounded-none";
    const imgDim = "w-[100px] h-[100px]";
    const animate = "animate-pulse";
    const imgError = placeholder;

    function castToCoverInfo(coverInfo: any): ICoverInfo {
        return castToType<ICoverInfo>(coverInfo);
    }

    onMount(async () => {
        if (!rels) {
            errored = true;
            return;
        }

        try {
            const coverRes = await getCaaCovers(rels[0].id);
            const coverData = await coverRes.json();
            frontImages = castToCoverInfo(coverData).images;

            if (frontImages.length == 0) {
                errored = true;
            }
        } catch {
            errored = true;
        }
    });
</script>

<div class="flex flex-col gap-2">
    <div class="flex mt-2 justify-start">
        <div class="flex items-center">
            {#if errored}
                <div class={classNames(imgError, imgDim)}></div>
            {:else if frontImages.length > 0}
                <img class={imgDim} alt="{relLog.title} by {relLog.artist}" src={frontImages[0].thumbnails.small} />
            {:else}
                <div class={classNames(placeholder, imgDim, animate)}></div>
            {/if}
            <div class="ml-4 flex flex-col">
                {#if rels.length > 0}
                    <!-- Only using the first release -->
                    <span class="text-md">{getJoinedArtists(rels[0]['artist-credit'])}</span>
                    <span class="text-2xl font-bold">{rels[0].title}</span>
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
            {#if rels.length == 1}
                <a type="button" class="self-end" href="https://musicbrainz.org/release/{rels[0].id}" target="_blank">
                    <span class="text-sm font-mono">{rels[0].id.split("-")[0]}</span>
                </a>
            {:else if rels.length > 1}
                <a type="button" class="self-end" href="https://musicbrainz.org/cdtoc/{mbzTocId}" target="_blank">
                    <span class="text-sm font-mono">MULTIPLE</span>
                </a>
            {:else if pending}
                <div class="{classNames(placeholder, animate)} h-4 w-72"></div>
            {:else}
                <span class="text-sm font-mono">N/A</span>
            {/if}
        </div>
    </div>
</div>