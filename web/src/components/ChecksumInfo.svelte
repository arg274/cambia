<script lang="ts">
	import Card from "./frags/Card.svelte";
    
	import type { Checksum } from "$lib/types/Checksum";
	import InfoSegment from "./frags/InfoSegment.svelte";
	import ChecksumSegment from "./frags/ChecksumSegment.svelte";
    import IconDoubleInteger from '~icons/carbon/double-integer';
    import IconCopyFile from '~icons/carbon/copy-file';

    export let checksum: Checksum;

</script>

<Card header="Checksum">
    <div class="flex flex-col gap-4">
        <InfoSegment header="Integrity" value={checksum.integrity} icon={IconDoubleInteger} />
        {#if checksum.log}
            <ChecksumSegment header="Log" hash={checksum.log} icon={IconCopyFile} status={checksum.integrity} />
        {/if}
    </div>    
</Card>