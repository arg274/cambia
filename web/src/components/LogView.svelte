<script lang="ts">
    import { Paginator, type PaginationSettings, tocCrawler } from '@skeletonlabs/skeleton';
    import classNames from 'classnames';
    
	import EvaluationInfo from "./EvaluationInfo.svelte";
	import Grade from "./Grade.svelte";
	import RipInfo from "./RipInfo.svelte";
	import RipInfoQuartet from "./RipInfoQuartet.svelte";
	import TocInfo from "./TocInfo.svelte";
	import ReleaseInfo from "./ReleaseInfo.svelte";
	import ChecksumInfo from "./ChecksumInfo.svelte";
	import TrackInfo from "./TrackInfo.svelte";
	import type { CambiaResponse } from '$lib/types/CambiaResponse';
    
    export let res: CambiaResponse;

    let pageSettings: PaginationSettings = {
        page: 0,
        limit: 1,
        size: res.parsed.parsed_logs.length,
        amounts: [1]
    } as PaginationSettings;
</script>

{#if res}
    {@const combinedLog = res.parsed.parsed_logs.length > 1 ? true : false}
    {@const parsedLog = res.parsed.parsed_logs[pageSettings.page]}
    {#if combinedLog}
        <div class="flex justify-between items-center">
            <span class="text-xs uppercase tracking-widest">Combined Log</span>
            <!-- TODO: Poor UX in paginators since individual pages cannot be jumped to -->
            <Paginator
                bind:settings={pageSettings}>
            </Paginator>
        </div>
    {/if}

    <div class={classNames("flex flex-col gap-y-4", combinedLog ? "mt-4" : "")}>
        <!-- <ReleaseInfo mbzTocId={parsedLog.toc.mbz.hash} /> -->
        <div class="hidden md:flex flex-col gap-y-4" use:tocCrawler={{ mode: 'generate', queryElements: 'h3' }}>
            <div class="hidden md:flex gap-x-4">
                <div class="flex flex-col w-1/2 gap-4">
                    {#key res.evaluation_combined}
                        <Grade evaluations={res.evaluation_combined} />
                    {/key}
                    <EvaluationInfo combinedEvals={res.evaluation_combined.filter(x => x.evaluator !== 'Cambia')} selectedLogIdx={pageSettings.page} />
                    <TocInfo toc={parsedLog.toc} />
                    <ChecksumInfo checksum={parsedLog.checksum} />
                </div>
                <div class="flex flex-col w-1/2 gap-4">
                    <RipInfo parsedLog={parsedLog} />
                    <RipInfoQuartet parsedLog={parsedLog} />
                </div>
            </div>
            <hr class="!border-t-4 !border-dashed" />
            <TrackInfo toc={parsedLog.toc.raw} tracks={parsedLog.tracks} />
        </div>
        <div class="flex md:hidden flex-col gap-4">
            {#key res.evaluation_combined}
                <Grade evaluations={res.evaluation_combined} />
            {/key}
            <EvaluationInfo combinedEvals={res.evaluation_combined.filter(x => x.evaluator !== 'Cambia')} selectedLogIdx={pageSettings.page} />
            <RipInfo parsedLog={parsedLog} />
            <RipInfoQuartet parsedLog={parsedLog} />
            <ChecksumInfo checksum={parsedLog.checksum} />
            <TocInfo toc={parsedLog.toc} />
            <TrackInfo toc={parsedLog.toc.raw} tracks={parsedLog.tracks} />
        </div>
    </div>
{/if}