<script lang="ts">
	import type { EvaluationCombined } from "$lib/types/EvaluationCombined";
	import type { ParsedLogCombined } from "$lib/types/ParsedLogCombined";
	import InfoOverviewUnit from "./frags/InfoOverviewUnit.svelte";

    export let parsedLogs: ParsedLogCombined;
    export let evalCombined: EvaluationCombined;

    const gradeMap = evalCombined.evaluations.map(e => {
        const m: Map<string, string> = new Map();
        e.evaluation_units.forEach(d => {
            m.set(d.data.field, d.unit_score);
        });
        return m;
    });
</script>

<div class="flex flex-col gap-2">
    {#each parsedLogs.parsed_logs as parsedLog, idx}
        {@const hideLabel = parsedLogs.parsed_logs.length != (idx + 1)}
        <div class="relative flex items-end h-8">
            <div class="mr-1 flex flex-col text-xxs font-mono leading-none">
                <span>T</span>
                <span>F</span>
                <span>?</span>
                <span>X</span>
            </div>

            <!-- Running this in a loop does not work for some reason -->
            <!-- There's some extra bit on the right -->
            <div class="absolute left-2 top-0 w-full border border-dashed border-surface-400 opacity-20"></div>
            <div class="absolute left-2 top-2 w-full border border-dashed border-surface-400 opacity-20"></div>
            <div class="absolute left-2 top-4 w-full border border-dashed border-surface-400 opacity-20"></div>
            <div class="absolute left-2 top-6 w-full border border-dashed border-surface-400 opacity-20"></div>
            
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='AccurateStream' actualValue={parsedLog.accurate_stream} miniName='ACS' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='Cache' actualValue={parsedLog.defeat_audio_cache} miniName='DAC' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='C2' actualValue={parsedLog.use_c2} miniName='C2E' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='Samples' actualValue={parsedLog.fill_silence} miniName='FMS' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='SilentBlocks' actualValue={parsedLog.delete_silence} miniName='DSB' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='NullSamples' actualValue={parsedLog.use_null_samples} miniName='NSC' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='TestAndCopy' actualValue={parsedLog.test_and_copy} miniName='T&C' />
            <InfoOverviewUnit gradeMap={gradeMap} index={idx} hideLabel={hideLabel} gradeKey='Normalization' actualValue={parsedLog.normalize} miniName='NML' />
        </div>
    {/each}
</div>