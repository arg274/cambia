<script lang="ts">
	import Card from "./frags/Card.svelte";
    
	import type { Checksum } from "$lib/types/Checksum";
	import InfoSegment from "./frags/InfoSegment.svelte";
	import ChecksumSegment from "./frags/ChecksumSegment.svelte";
    import IconDoubleInteger from '~icons/carbon/double-integer';
    import IconJoinOuter from '~icons/carbon/join-outer';
    import IconCalculator from '~icons/carbon/calculator';
    import IconCopyFile from '~icons/carbon/copy-file';

    export let checksum: Checksum;

</script>

<Card header="Checksum" addClass="grow">
    <div class="flex flex-col gap-4">
        <InfoSegment header="Integrity" value={checksum.integrity} icon={IconDoubleInteger} />
        {#if checksum.integrity === 'Match' }
            <ChecksumSegment header="Log + Calculated" hash={checksum.log} icon={IconJoinOuter} status={checksum.integrity} />
        {:else}
            {#if checksum.calculated}
                <ChecksumSegment header="Calculated" hash={checksum.calculated} icon={IconCalculator} status={checksum.integrity} />
            {/if}
            {#if checksum.log}
                <ChecksumSegment header="Log" hash={checksum.log} icon={IconCopyFile} status={checksum.integrity} />
            {/if}
        {/if}
    </div>    
</Card>