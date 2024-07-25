<script lang="ts">
    import type { TrackEntry } from "$lib/types/TrackEntry";
	import AccurateRipChecksumSegment from "./frags/AccurateRipChecksumSegment.svelte";
	import InfoSegment from "./frags/InfoSegment.svelte";

    export let track: TrackEntry;

    $: ar = track.ar_info[track.ar_info.length - 1];
</script>
<div class="flex flex-col gap-4">
    <div class="flex gap-2 items-center">
        <h6>Track {track.num}</h6>
        {#if track.aborted}
            <div class="variant-soft-error rounded-md text-xs px-2 py-1">Aborted</div>
        {/if}
    </div>
    <div class="flex flex-col gap-4">
        <InfoSegment header="Status" value={ar.status} />
        <AccurateRipChecksumSegment header="Signature" sign={ar.sign} />
        {#if ar.sign !== ar.offset_sign}
            <AccurateRipChecksumSegment header="Signature" sign={ar.offset_sign} arOffset={ar.confidence?.offset} />
        {/if}
    </div>
</div>