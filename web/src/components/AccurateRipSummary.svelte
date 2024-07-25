<script lang="ts">
    import type { TrackEntry } from "$lib/types/TrackEntry";
    import type { AccurateRipStatus } from "$lib/types/AccurateRipStatus";
	import Card from "./frags/Card.svelte";
	import AccurateRipInfoText from "./AccurateRipInfoText.svelte";

    export let tracks: TrackEntry[];
    $: selectedTrack = 0;
    
    const boxSize = 8;
    const n = Math.ceil(Math.sqrt(tracks.length));
    const n_sq = n * n;

    function getColor(status?: AccurateRipStatus) {
        switch (status) {
            case "Match": return "bg-primary-400";
            case "Mismatch": return "bg-error-400";
            case "Offsetted": return "bg-tertiary-400";
            case "NotFound": return "bg-warning-400";
            case "Disabled": return "bg-surface-200";
            default: return "bg-surface-600";
        }
    }

    function getText(status?: AccurateRipStatus, track?: number) {
        switch (status) {
            case "Mismatch":
            case "Offsetted":
            case "NotFound":
            case "Disabled":
                return track;
            default: return "";
        }
    }

    function gridGenerator(cols: number): string {
        return `grid-template-columns: repeat(${cols}, minmax(0, 1fr));`
    }

    function cubeGenerator(cols: number): string {
        return `width: ${boxSize / n}rem; height: ${boxSize / n}rem;`
    }

    function clickHandler(idx: number) {
        if (idx < tracks.length) {
            selectedTrack = idx;
        }
    }
</script>

<Card header="AccurateRip Summary">
    <!-- TODO: Check if the styles can be done via TW -->
    <!-- TODO: Add a tab group for v1/v2 -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
        <div class="grid place-self-start self-center md:justify-self-center" style={gridGenerator(n)}>
            {#each Array(n_sq) as _, trackIdx}
                {@const ars = tracks[trackIdx]?.ar_info}
                {@const ar = ars ? ars[ars.length - 1] : undefined}
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <div class="{getColor(ar?.status)} hover:bg-success-400 hover:cursor-pointer text-center text-xs content-center"
                    style={cubeGenerator(n)}
                    on:keydown={Function.prototype()}
                    on:click={() => {clickHandler(trackIdx)}}>
                </div>
            {/each}
        </div>
        <AccurateRipInfoText track={tracks[selectedTrack]} />
    </div>
</Card>