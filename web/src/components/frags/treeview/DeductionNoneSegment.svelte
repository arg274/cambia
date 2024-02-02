<script lang="ts">
	import type { Checksum } from "$lib/types/Checksum";
	import Explainable from "../Explainable.svelte";

    export let checksum: Checksum;
</script>

<div class="text-xs py-2">
    {#if checksum.integrity === "Match"}
        This rip log is expected to be perfect and untrumpable by <Explainable explanation="A private tracker">Orpheus</Explainable> standards.
    {:else if checksum.integrity === "Unknown"}
        This rip log is expected to be perfect but trumpable by <Explainable explanation="A private tracker">Orpheus</Explainable> standards due to the checksum being missing.
    {:else}
        This rip log has been tampered with, as the checksum does not match.
    {/if}
</div>