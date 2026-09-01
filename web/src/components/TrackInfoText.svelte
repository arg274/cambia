<script lang="ts">
    import { untrack } from 'svelte';

	import { Accordion } from "@skeletonlabs/skeleton-svelte";
    import Paginator from "./frags/Paginator.svelte";
    import IconSplitScreen from '~icons/carbon/split-screen';
    import IconIncompleteCancel from '~icons/carbon/incomplete-cancel';
    import IconSidePanelOpenFilled from '~icons/carbon/side-panel-open-filled';
    import IconMountain from '~icons/carbon/mountain';
    import IconMicrophone from '~icons/carbon/microphone';
    import IconExpandCategories from '~icons/carbon/expand-categories';
    import IconTransmissionLte from '~icons/carbon/transmission-lte';
    import IconDocumentBlank from '~icons/carbon/document-blank';
    import IconDoubleInteger from '~icons/carbon/double-integer';
    import IconHashtag from '~icons/carbon/hashtag';
    import IconWarningAlt from '~icons/carbon/warning-alt';
    import IconCheckmarkFilledError from '~icons/carbon/checkmark-filled-error';
    import IconArrowRight from '~icons/carbon/arrow-right';
	import ChecksumSegment from "./frags/ChecksumSegment.svelte";
	import InfoSegment from "./frags/InfoSegment.svelte";
	import type { TrackEntry } from "$lib/types/TrackEntry";
	import { nonNullAssert, secondsToMMSS, trimLeftChar } from "$lib/utils";
    
    interface Props {
        tracks: TrackEntry[];
        selectedTrack: number;
    }

    let { tracks, selectedTrack = $bindable() }: Props = $props();

    // TODO: Lots of functionality duped from LogView paginator

    let pageIndex = $state(0);
    const pageCount = $derived(tracks.length);
    
    let outerTrack = $derived(selectedTrack);
    let inputPage = $state(selectedTrack);
    let inputEl: HTMLInputElement | undefined = $state();

    function onPageChange() {
        inputPage = pageIndex + 1;
    }

    function pageInputHandler(ev: KeyboardEvent) {
        switch (ev.key) {
            case ",":
            case ".":
            case "-":
            case "e":
                ev.preventDefault();
                break;
            case "Enter":
                ev.preventDefault();
                gotoPage();
                break;
        }
    }

    function gotoPage() {
        if (typeof inputPage !== 'number' || isNaN(inputPage)) {
            return;
        }
        const trunc = Math.ceil(inputPage) - 1;
        if (trunc < 0 || trunc >= pageCount) {
            return;
        }
        pageIndex = trunc;
    }

    function selectText() {
        inputEl?.select();
    }
    
    $effect(() => {
        const track = outerTrack;
        untrack(() => {
            inputPage = track;
            gotoPage();
        });
    });
</script>
{#if tracks.length > 0}
    <div>
        <div class="flex justify-center md:justify-end md:items-center items-end gap-2 mb-2">
            <Paginator count={pageCount} bind:page={pageIndex} {onPageChange} />
            <div class="flex justify-end items-stretch hide-scroll-numinput">
                <input type="number" required bind:value={inputPage} class="w-12 preset-filled py-1.5 text-center text-sm rounded-l-full" onkeypress={pageInputHandler} onclick={(e) => { e.preventDefault(); selectText(); }} bind:this={inputEl} />
                <button type="button" class="preset-filled py-1.5 px-2 text-sm rounded-r-full" onclick={gotoPage}><IconArrowRight class="w-3.5 h-3.5" /></button>
            </div>
        </div>
        <div class="flex flex-col gap-4">
            <div class="flex gap-2 items-center">
                <h6>Track {tracks[pageIndex].num}</h6>
                {#if tracks[pageIndex].aborted}
                    <div class="preset-tonal-error rounded-md text-xs px-2 py-1">Aborted</div>
                {/if}
            </div>
            <div class="flex flex-col gap-4">
                <InfoSegment icon={IconSplitScreen} header="Track splitting" value={tracks[pageIndex].is_range ? "Range" : "Split"} />
                <InfoSegment icon={IconSidePanelOpenFilled} header="Extraction speed" value={`${tracks[pageIndex].extraction_speed?.toFixed(1)}x`} />
                <InfoSegment icon={IconMountain} header="Peak level" value={tracks[pageIndex].peak_level?.toFixed(3)} />
                <InfoSegment icon={IconMicrophone} header="Gain" value={tracks[pageIndex].gain} />
                <InfoSegment icon={IconExpandCategories} header="Pregap length" value={tracks[pageIndex].pregap_length ? `${parseFloat(nonNullAssert(tracks[pageIndex].pregap_length)).toFixed(2)} sec` : null} />
                <InfoSegment icon={IconTransmissionLte} header="Pre-emphasis" value={tracks[pageIndex].preemphasis} />
            </div>
            {#if tracks[pageIndex].filenames.length > 0}
                <div class="flex flex-col gap-2">   
                    <div class="flex items-center"><IconDocumentBlank class="icon-sm" /><span class="ml-2 dark:font-light text-sm">Filename</span></div>
                    <!-- TODO: Only show the first filename for now -->
                    <!-- Leading slashes in *nix paths need to be trimmed to not mess up RTL -->
                    <div class="font-mono grow bg-surface-50-950 px-2 py-1 col-span-9 truncate text-end" dir="rtl">{trimLeftChar(tracks[pageIndex].filenames[0], "/")}</div>
                </div>
            {/if}
            <hr class="!border-t-4 !border-dashed" />
            <InfoSegment header="Integrity" value={tracks[pageIndex].test_and_copy.integrity} icon={IconDoubleInteger} />
            <InfoSegment header="Integrity (skip zeroes)" value={tracks[pageIndex].test_and_copy.integrity_skipzero} icon={IconDoubleInteger} />
            
            {#if tracks[pageIndex].test_and_copy.integrity === 'Match'}
                <ChecksumSegment header="T&C hash" hash={tracks[pageIndex].test_and_copy.test_hash} icon={IconHashtag} status={tracks[pageIndex].test_and_copy.integrity} />
            {:else}
                <ChecksumSegment header="Test hash" hash={tracks[pageIndex].test_and_copy.test_hash} icon={IconHashtag} status={tracks[pageIndex].test_and_copy.integrity} />
                <ChecksumSegment header="Copy hash" hash={tracks[pageIndex].test_and_copy.copy_hash} icon={IconHashtag} status={tracks[pageIndex].test_and_copy.integrity} />
            {/if}

            {#if tracks[pageIndex].test_and_copy.integrity_skipzero === 'Match'}
                <ChecksumSegment header="T&C hash (skip zeroes)" hash={tracks[pageIndex].test_and_copy.test_skipzero_hash} icon={IconHashtag} status={tracks[pageIndex].test_and_copy.integrity_skipzero} />
            {:else}
            <ChecksumSegment header="Test hash (skip zeroes)" hash={tracks[pageIndex].test_and_copy.test_skipzero_hash} icon={IconHashtag} status={tracks[pageIndex].test_and_copy.integrity_skipzero} />
            <ChecksumSegment header="Copy hash (skip zeroes)" hash={tracks[pageIndex].test_and_copy.copy_skipzero_hash} icon={IconHashtag} status={tracks[pageIndex].test_and_copy.integrity_skipzero} />
            {/if}
        </div>
        {#if Object.keys(tracks[pageIndex].errors).length > 0}
            <hr class="my-4 !border-t-4 !border-dashed" />
            <div class="flex items-center"><IconWarningAlt /><span class="ml-1 dark:font-light text-sm">Track Errors</span></div>
            <Accordion multiple collapsible class="mt-2 space-y-0">
                {#each Object.keys(tracks[pageIndex].errors) as errorType (errorType)}
                    <Accordion.Item value={errorType}>
                        <Accordion.ItemTrigger class="w-full flex items-center gap-2 px-2 py-1">
                            <IconCheckmarkFilledError />
                            <div class="grow flex justify-between items-center">
                                <span class="first-letter:capitalize text-sm">{errorType}</span>
                                <span class="chip preset-tonal-error rounded-full">{tracks[pageIndex].errors[errorType].count}</span>
                            </div>
                        </Accordion.ItemTrigger>
                        <Accordion.ItemContent class="px-2 py-1">
                            {#if tracks[pageIndex].errors[errorType].ranges.length > 0}
                                {#each tracks[pageIndex].errors[errorType].ranges as errorRange}
                                    <div class="flex justify-between items-center">
                                        <div>
                                            <span class="text-xs preset-tonal-primary rounded-full py-1 px-2 uppercase">Start</span>
                                            <span class="text-xs">{secondsToMMSS(parseFloat(errorRange.start))}</span>
                                        </div>
                                        {#if errorRange.length}
                                            <hr class="mx-2 grow !border-b-2 !border-dotted" />
                                            <div>
                                                <span class="text-xs">{secondsToMMSS(parseFloat(errorRange.start) + parseFloat(errorRange.length))}</span>
                                                <span class="text-xs preset-tonal-primary rounded-full py-1 px-2 uppercase">End</span>
                                            </div>
                                        {/if}
                                    </div>
                                {/each}
                            {:else}
                                <span class="text-xs">Position data not available/applicable.</span>
                            {/if}
                        </Accordion.ItemContent>
                    </Accordion.Item>
                {/each}
            </Accordion>
        {/if}
    </div>
{/if}