<script lang="ts">
	import type { ParsedLog } from "$lib/types/ParsedLog";
    import IconCarouselVertical from '~icons/carbon/carousel-vertical';
    import IconChartAreaSmooth from '~icons/carbon/chart-area-smooth';
    import IconHashtag from '~icons/carbon/hashtag';
    import IconHealthCross from '~icons/carbon/health-cross';
    import IconTagEdit from '~icons/carbon/tag-edit';
    import IconVolumeMute from '~icons/carbon/volume-mute';
    import IconVolumeUpAlt from '~icons/carbon/volume-up-alt';
    import IconCdCreateArchive from '~icons/carbon/cd-create-archive';
    import IconRegistration from '~icons/carbon/registration';
    import IconWindStream from '~icons/carbon/wind-stream';
	import Card from "./frags/Card.svelte";
	import InfoSegmentQuartet from "./frags/InfoSegmentQuartet.svelte";

    export let parsedLog: ParsedLog;
</script>

<Card header="Rip Settings" addClass="grow">
    <div class="flex flex-col striped">
        <InfoSegmentQuartet header="Accurate Stream" value={parsedLog.accurate_stream} icon={IconWindStream} />
        <InfoSegmentQuartet header="Defeat Audio Cache" value={parsedLog.defeat_audio_cache} icon={IconCdCreateArchive} />
        <InfoSegmentQuartet header="Use C2" value={parsedLog.use_c2} icon={IconHealthCross} />
        <InfoSegmentQuartet header="Overread" value={parsedLog.overread} icon={IconCarouselVertical} />
        <InfoSegmentQuartet header="Fill Missing Samples" value={parsedLog.fill_silence} icon={IconVolumeUpAlt} />
        <InfoSegmentQuartet header="Delete Leading/Trailing Silence" value={parsedLog.delete_silence} icon={IconVolumeMute} />
        <InfoSegmentQuartet header="Null Samples in CRC" value={parsedLog.use_null_samples} icon={IconHashtag} />
        <InfoSegmentQuartet header="Test and Copy" value={parsedLog.test_and_copy} icon={IconRegistration} />
        <InfoSegmentQuartet header="Normalise" value={parsedLog.normalize} icon={IconChartAreaSmooth} />
        <InfoSegmentQuartet header="Add ID3" value={parsedLog.id3_enabled} icon={IconTagEdit} />
    </div>
</Card>