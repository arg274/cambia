<script lang="ts">
    import Color from 'colorjs.io';

	import type { TocRaw } from "$lib/types/TocRaw";
	import type { TrackEntry } from "$lib/types/TrackEntry";
	import Card from "./frags/Card.svelte";
	import type { SegmentDetails } from '$lib/types/SegmentDetails';
	import type { LaneDetails } from '$lib/types/LaneDetails';
	import { getCssColor, getCssColorHex } from '$lib/utils';
	import classNames from 'classnames';
	import TrackInfoText from './TrackInfoText.svelte';

    export let toc: TocRaw;
    export let tracks: TrackEntry[];

    $: selectedTrack = 1;
    $: trackLookupMap = getTrackLookup(tracks);
    $: rippedTracks = Array.from(trackLookupMap.keys());
    
    function getTrackLookup(trackEntries: TrackEntry[]): Map<number, number> {
        const trackMap = new Map<number, number>();

        trackEntries.forEach((entry, index) => {
            trackMap.set(entry.num, index);
        });

        return trackMap;
    }

    function getBlockNum(i: number): number {
        let laneArea = Math.PI * (((innerRadius + (i + 1) * gapRadius) ** 2) - ((innerRadius + i * gapRadius) ** 2))
        return Math.round(laneArea / segArea);
    }

    function getBound(num: number, reverse: boolean = false): number | null {
        let bound = reverse ? startMinutes.filter(x => x <= num) : endMinutes.filter(x => x >= num);
        if (bound.length === 0) {
            return null;
        }
        return reverse ? Math.max(...bound) : Math.min(...bound);
    }

    function getTracksFromMinute(minute: number, reverse: boolean = false): number[] {
        // HTOA
        if (minute < startMinutes[0]) {
            return [-1];
        }

        let bound = getBound(minute, reverse);
        if (bound === null) {
            return [9998];
        }
        return reverse ? reverseLookupStart.get(bound)! : reverseLookupEnd.get(bound)!;
    }

    function getTrackColor(segmentNum: number): Color {
        // HTOA
        if (segmentNum < startMinutes[0]) {
            return getCssColor("color-warning-400");
        }

        let bound = getBound(segmentNum);
        if (bound === null) {
            return getCssColor("color-surface-600");
        }

        let localTracks = getTracksFromMinute(segmentNum);

        // Data tracks
        if (toc.data_tracks > 0) {
            let dataTracks = localTracks.filter(x => ((x > (tracks.length - 1) && x !== 9998)));
            if (dataTracks.length === 1) {
                return getCssColor("color-tertiary-400");
            } else if (dataTracks.length > 1) {
                // DRM
                return getCssColor("color-tertiary-600");
            }
        }
        
        for (let idx = 0; idx < localTracks.length; idx++) {
            // Not selected during rip
            if (!rippedTracks.includes(localTracks[idx] + 1)) {
                return getCssColor("color-surface-200");
            }
            const orig_idx = trackLookupMap.get(localTracks[idx] + 1)!;
            // Aborted
            if (tracks[orig_idx].aborted) {
                return getCssColor("color-surface-200");
            }
            // T&C
            if (tracks[orig_idx].test_and_copy.integrity === "Mismatch" || tracks[orig_idx].test_and_copy.integrity_skipzero === "Mismatch") {
                return errorGradient[orig_idx];
            }
            if (tracks[orig_idx].test_and_copy.integrity === "Unknown" && tracks[orig_idx].test_and_copy.integrity_skipzero === "Unknown") {
                return getCssColor("color-warning-600");
            }
            // Errors
            let errorCount = Object.keys(tracks[orig_idx].errors).length;
            if (errorCount > 0) {
                return errorGradient[orig_idx];
            }
        }
        return gradient[endMinutes.indexOf(bound)];
    }

    function getSegmentText(segmentNum: number) {
        // HTOA
        if (segmentNum === 0 && segmentNum < startMinutes[0]) {
            return "HT";
        }

        if (!endMinutes.includes(segmentNum)) {
            return "";
        }

        let localTracks = getTracksFromMinute(segmentNum);

        // Data tracks
        if (toc.data_tracks > 0) {
            let dataTracks = localTracks.filter(x => ((x > (tracks.length - 1) && x !== 9998)));
            if (dataTracks.length === 1) {
                return "DAT";
            } else if (dataTracks.length > 1) {
                // DRM
                return "DRM";
            }
        }

        return (localTracks.length > 1) ? "TM" : `T${localTracks[0] + 1}`;
    }

    let reverseLookupEnd: Map<number, number[]>;
    let reverseLookupStart: Map<number, number[]>;

    let endMinutes: number[];
    let startMinutes: number[];

    let vb: number;
    let outerRadius: number;
    let innerRadius: number;
    let laneCount: number;
    let minutes: number;
    let gapRadius: number;
    let segArea: number;
    
    let startColor: Color = getCssColor("color-primary-300");
    let endColor: Color = getCssColor("color-primary-700");
    let errorStartColor: Color = getCssColor("color-error-300");
    let errorEndColor: Color = getCssColor("color-error-700");

    let gradient: Color[];
    let errorGradient: Color[];

    let lanes: LaneDetails[];

    $: {
        reverseLookupEnd = new Map();
        reverseLookupStart = new Map();

        for (let idx = 0; idx < toc.entries.length; idx++) {
            let startMinute = Math.round(toc.entries[idx].start_sector / (75 * 60));
            let endMinute = Math.round(toc.entries[idx].end_sector / (75 * 60));
            reverseLookupEnd.has(endMinute) ? reverseLookupEnd.get(endMinute)!.push(idx) : reverseLookupEnd.set(endMinute, [idx]);
            reverseLookupStart.has(startMinute) ? reverseLookupStart.get(startMinute)!.push(idx) : reverseLookupStart.set(startMinute, [idx]);
        }

        endMinutes = Array.from(reverseLookupEnd.keys());
        startMinutes = Array.from(reverseLookupStart.keys());

        vb = 36;
        outerRadius = 100 / (2 * Math.PI);
        innerRadius = 5;
        laneCount = 5;
        minutes = Math.max(Math.ceil(toc.lead_out / (75 * 60)), 80);
        gapRadius = (outerRadius - innerRadius) / laneCount;

        // Segment count might not always be exact always due to rounding issues
        // Adding 1 so that it doesn't underestimate
        segArea = (Math.PI * (outerRadius ** 2 - innerRadius ** 2)) / (minutes + 1);

        gradient = Color.steps(startColor, endColor, {
                                space: "srgb",
                                outputSpace: "srgb",
                                steps: endMinutes.length,
                            }).map(plainColor => new Color(plainColor));
        errorGradient = Color.steps(errorStartColor, errorEndColor, {
                                    space: "srgb",
                                    outputSpace: "srgb",
                                    steps: tracks.length,
                                }).map(plainColor => new Color(plainColor));

        let currentMinute = 0;

        lanes = [];

        for (let l = 0; l < laneCount; l++) {
            let blocks = getBlockNum(l);
            let segments: SegmentDetails[] = [];
            let radius = innerRadius + (l * gapRadius);
            let perimeter = 2 * Math.PI * radius;
            

            for (let b = 0; b < blocks; b++) {
                let unitAngle = 2 * Math.PI / blocks;
                let trackColor = getTrackColor(currentMinute);

                let darkContrast = trackColor.contrastWCAG21(getCssColor("color-tertiary-900"));
                let lightContrast = trackColor.contrastWCAG21(getCssColor("color-tertiary-50"));
                
                segments.push({
                    minute: currentMinute,
                    text: getSegmentText(currentMinute),
                    color: getTrackColor(currentMinute).toString({format: "hex"}),
                    textColor: darkContrast > lightContrast ? getCssColorHex("color-tertiary-900") : getCssColorHex("color-tertiary-50"),
                    offset: (perimeter * (0.25 + (1 / blocks))) + (b * perimeter * (1 / blocks)),
                    x: radius * Math.cos((unitAngle * (blocks - b)) - (Math.PI / 2) - unitAngle + (unitAngle / 2)),
                    y: radius * Math.sin((unitAngle * (blocks - b)) - (Math.PI / 2) - unitAngle + (unitAngle / 2)),
                    trackIndices: getTracksFromMinute(currentMinute),
                });
                currentMinute++;
            }
            lanes.push({
                radius: radius,
                segmentSize: perimeter * (1 / blocks),
                segmentGap: perimeter * (1 - (1 / blocks)),
                segments: segments
            });
        }
    }

    function clickHandler(segment: SegmentDetails) {
        const idx = trackLookupMap.get(segment.trackIndices[0] + 1);
        if (idx !== undefined) {
            selectedTrack = idx + 1;
        }
    }
</script>

<style>
    svg {
        pointer-events: visibleStroke;
    }
</style>

<!-- FIXME: Abysmal UX when accordion expands -->
<!-- TODO: This addClass is a freakish hack for popups not getting covered -->
<Card header="Track Details" addClass="!z-0">
    <!-- Rendering might be broken in Chrome if crispEdges is not used in some cases -->
    <div class="grid grid-cols-1 md:grid-cols-2 pb-4">
        <svg class="mb-4" width="100%" height="100%" viewBox="0 0 {vb} {vb}">
            <circle class="stroke-surface-400" cx="{vb / 2}" cy="{vb / 2}" r="{((outerRadius + innerRadius) / 2) - (gapRadius / 2)}" stroke-width="{outerRadius - innerRadius}" fill="transparent"></circle>
            {#if toc.entries.length > 0}
                {#each lanes as lane}
                    {#each lane.segments as segment}
                        <!-- svelte-ignore a11y-no-static-element-interactions -->
                        <circle id="minute-{segment.minute}" class={classNames(segment.trackIndices.map(trackIdx => `track-${trackIdx + 1}`), "cursor-pointer hover:stroke-success-400")} cx="{vb / 2}" cy="{vb / 2}" r="{lane.radius}" fill="transparent"
                            stroke="{segment.color}" stroke-width="{gapRadius}"
                            stroke-dasharray="{lane.segmentSize} {lane.segmentGap}"
                            stroke-dashoffset="{segment.offset}"
                            on:keydown={Function.prototype()}
                            on:click={() => clickHandler(segment)}></circle>
                        <text
                            class="pointer-events-none"
                            fill="{segment.textColor}"
                            x={(vb / 2) + segment.x}
                            y={(vb / 2) + segment.y}
                            alignment-baseline="middle"
                            text-anchor="middle"
                            font-size="1">
                            {segment.text}
                        </text>
                    {/each}
                {/each}
            {:else}
                <text class="fill-surface-900 dark:fill-surface-50" x={vb / 2} y={vb} font-size="2" alignment-baseline="middle" text-anchor="middle">No TOC found</text>
            {/if}
        </svg>
        {#key toc}
            <TrackInfoText tracks={tracks} bind:selectedTrack />
        {/key}
    </div>
</Card>