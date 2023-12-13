<script lang="ts">
    import IconVersion from '~icons/carbon/version';
    import IconCdCreateExchange from '~icons/carbon/cd-create-exchange';
    import IconRadar from '~icons/carbon/radar';
    import IconIbmWatsonLanguageTranslator from '~icons/carbon/ibm-watson-language-translator';
    import IconWorkspace from '~icons/carbon/workspace';
    import IconShowDataCards from '~icons/carbon/show-data-cards';
    import IconDocumentWordProcessorReference from '~icons/carbon/document-word-processor-reference';
    import IconCdArchive from '~icons/carbon/cd-archive';
	import InfoSegment from './frags/InfoSegment.svelte';
	import Card from './frags/Card.svelte';
	import type { ParsedLog } from "$lib/types/ParsedLog";

    export let parsedLog: ParsedLog;
</script>

<Card header="Rip Info">
    <div class="flex flex-col gap-4">
        <div class="flex gap-2 items-end">
            <InfoSegment icon={IconCdCreateExchange} header="Ripper" value={parsedLog.ripper} />
            <div class="variant-soft-primary rounded-md text-sm px-2 py-1 font-mono">v{parsedLog.ripper_version}</div>
        </div>
        <InfoSegment icon={IconDocumentWordProcessorReference} header="Read mode" value={parsedLog.read_mode} />
        <InfoSegment icon={IconRadar} header="Read offset" value={parsedLog.read_offset && parsedLog.read_offset > 0 ? "+" : ""}{parsedLog.read_offset} />
        <InfoSegment icon={IconRadar} header="Combined R/W offset" value={parsedLog.combined_rw_offset} />
        <InfoSegment icon={IconIbmWatsonLanguageTranslator} header="Language" value={parsedLog.language} />
        <InfoSegment icon={IconWorkspace} header="Gap handling" value={parsedLog.gap_handling} />
        <InfoSegment icon={IconShowDataCards} header="Drive" value={parsedLog.drive} />
        <InfoSegment icon={IconCdArchive} header="Media type" value={parsedLog.media_type} />
    </div>
</Card>