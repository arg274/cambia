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
    import IconCheckmarkFilled from '~icons/carbon/checkmark-filled';
    import IconCloseFilled from '~icons/carbon/close-filled';
	import Card from "./frags/Card.svelte";
	import InfoSegmentQuartet from "./frags/InfoSegmentQuartet.svelte";
	import type { Evaluation } from "$lib/types/Evaluation";
	import type { DeductionField } from "$lib/types/DeductionField";
	import { quartetToVariant } from "$lib/utils";

    export let parsedLog: ParsedLog;
    export let evaluation: Evaluation;

    let evMap: Record<DeductionField, number> = {
		Encoding: 0,
		RipperVersion: 0,
		Drive: 0,
		Ripper: 0,
		Offset: 0,
		Cache: 0,
		TestAndCopy: 0,
		Encoder: 0,
		Checksum: 0,
		MediaType: 0,
		ReadMode: 0,
		MaxRetryCount: 0,
		AccurateStream: 0,
		C2: 0,
		SilentSamples: 0,
		NullSamples: 0,
		Gap: 0,
		Tag: 0,
		Gain: 0,
		RangeSplit: 0,
		Samples: 0,
		SilentBlocks: 0,
		Normalization: 0,
		Filename: 0,
		ReadError: 0,
		SkipError: 0,
		JitterGenericError: 0,
		JitterEdgeError: 0,
		JitterAtomError: 0,
		DriftError: 0,
		DroppedError: 0,
		DuplicatedError: 0,
		InconsistentErrorSectors: 0,
		DamagedSector: 0,
		Abort: 0
	};
    evaluation.deductions.forEach((deduction) => {
        if (deduction.data.category === "Release") {
            const { field } = deduction.data;
            evMap[field] = evMap[field] + 1;
        }
    });
</script>

<Card header="Rip Settings">
    <div slot="tooltip" class="flex flex-col gap-2">
        <h6 class="text-lg font-bold">Actual values</h6>
        <div class="flex flex-col gap-2 text-xs">
            <p class="max-w-64 text-balanced">Actual values of a setting are shown on the left.</p>
            <div class="flex gap-2 items-center">
                <div class="w-1 h-4 rounded-full {quartetToVariant("True")}"></div>
                <span>Enabled</span>
            </div>
            <div class="flex gap-2 items-center">
                <div class="w-1 h-4 rounded-full {quartetToVariant("False")}"></div>
                <span>Disabled</span>
            </div>
            <div class="flex gap-2 items-center">
                <div class="w-1 h-4 rounded-full {quartetToVariant("Unknown")}"></div>
                <span>Unknown</span>
            </div>
            <div class="flex gap-2 items-center">
                <div class="w-1 h-4 rounded-full {quartetToVariant("Unsupported")}"></div>
                <span>Unsupported/Inapplicable</span>
            </div>
        </div>
        <h6 class="text-lg font-bold">Recommendation</h6>
        <div class="flex flex-col gap-2 text-xs">
            <p class="max-w-64 text-balanced">The ripper settings should be adjusted to ensure that all the values match the recommendation shown on the right.</p>
            <div class="flex gap-2 items-center">
                <IconCheckmarkFilled class="text-success-700 visible dark:text-success-400 icon-sm" />
                <span>Matches recommended value</span>
            </div>
            <div class="flex gap-2 items-center">
                <IconCloseFilled class="text-error-700 dark:text-error-400 icon-sm" />
                <span>Not the recommended value</span>
            </div>
        </div>
    </div>
    <div class="flex flex-col striped">
        <InfoSegmentQuartet header="Accurate stream" value={parsedLog.accurate_stream} valueOk={evMap["AccurateStream"]} icon={IconWindStream} />
        <InfoSegmentQuartet header="Defeat audio cache" value={parsedLog.defeat_audio_cache} valueOk={evMap["Cache"]} icon={IconCdCreateArchive} />
        <InfoSegmentQuartet header="Use C2" value={parsedLog.use_c2} valueOk={evMap["C2"]} icon={IconHealthCross} />
        <InfoSegmentQuartet header="Overread" value={parsedLog.overread} icon={IconCarouselVertical} />
        <InfoSegmentQuartet header="Fill missing samples" value={parsedLog.fill_silence} valueOk={evMap["Samples"]} icon={IconVolumeUpAlt} />
        <InfoSegmentQuartet header="Delete leading/trailing silence" value={parsedLog.delete_silence} valueOk={evMap["SilentBlocks"]} icon={IconVolumeMute} />
        <InfoSegmentQuartet header="Null samples in CRC" value={parsedLog.use_null_samples} valueOk={evMap["NullSamples"]} icon={IconHashtag} />
        <InfoSegmentQuartet header="Test and copy" value={parsedLog.test_and_copy} valueOk={evMap["TestAndCopy"]} icon={IconRegistration} />
        <InfoSegmentQuartet header="Normalise" value={parsedLog.normalize} valueOk={evMap["Normalization"]} icon={IconChartAreaSmooth} />
        <InfoSegmentQuartet header="Add ID3" value={parsedLog.id3_enabled} valueOk={evMap["Tag"]} icon={IconTagEdit} />
    </div>
</Card>