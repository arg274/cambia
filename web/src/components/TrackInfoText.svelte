<script lang="ts">
	import { Paginator, Accordion, AccordionItem, type PaginationSettings } from "@skeletonlabs/skeleton";
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
	import { nonNullAssert, trimLeftChar } from "$lib/utils";
    
    export let tracks: TrackEntry[];
    export let selectedTrack: number;

    // TODO: Lots of functionality duped from LogView paginator

    let page = {
        page: 0,
        limit: 1,
        size: tracks.length,
        amounts: [1]
    } as PaginationSettings;
    
    $: outerTrack = selectedTrack;
    let inputPage = selectedTrack;
    let inputEl: HTMLInputElement;

    function onPageChange() {
        inputPage = page.page + 1;
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
        if (trunc < 0 || trunc >= page.size) {
            return;
        }
        page = {
            page: trunc,
            limit: 1,
            size: tracks.length,
            amounts: [1]
        }
    }

    function selectText(e: MouseEvent) {
        inputEl.select();
    }
    
    $: {
        inputPage = outerTrack;
        gotoPage();
    }
</script>
<div>
    <div class="flex justify-center md:justify-end md:items-center items-end gap-2 mb-2">
        <Paginator bind:settings={page} on:page={onPageChange}></Paginator>
        <div class="flex justify-end items-center hide-scroll-numinput">
            <input type="number" required bind:value={inputPage} class="w-12 variant-filled py-1.5 text-center text-sm rounded-l-full" on:keypress={pageInputHandler} on:click|preventDefault={selectText} bind:this={inputEl} />
            <button type="button" class="variant-filled py-1.5 px-2 rounded-r-full" on:click={gotoPage}><IconArrowRight /></button>
        </div>
    </div>
    <div class="flex flex-col gap-4">
        <div class="flex gap-2 items-center">
            <h6>Track {tracks[page.page].num}</h6>
            {#if tracks[page.page].aborted}
                <div class="variant-soft-error rounded-md text-xs px-2 py-1">Aborted</div>
            {/if}
        </div>
        <div class="flex flex-col gap-4">
            <InfoSegment icon={IconSplitScreen} header="Track splitting" value={tracks[page.page].is_range ? "Range" : "Split"} />
            <InfoSegment icon={IconSidePanelOpenFilled} header="Extraction speed" value={`${tracks[page.page].extraction_speed?.toFixed(1)}x`} />
            <InfoSegment icon={IconMountain} header="Peak level" value={tracks[page.page].peak_level?.toFixed(3)} />
            <InfoSegment icon={IconMicrophone} header="Gain" value={tracks[page.page].gain} />
            <InfoSegment icon={IconExpandCategories} header="Pregap length" value={tracks[page.page].pregap_length ? `${parseFloat(nonNullAssert(tracks[page.page].pregap_length)).toFixed(2)} sec` : null} />
            <InfoSegment icon={IconTransmissionLte} header="Pre-emphasis" value={tracks[page.page].preemphasis} />
        </div>
        {#if tracks[page.page].filenames.length > 0}
            <div class="flex flex-col gap-2">   
                <div class="flex items-center"><IconDocumentBlank class="icon-sm" /><span class="ml-1 dark:font-light text-sm">Filename</span></div>
                <!-- TODO: Only show the first filename for now -->
                <!-- Leading slashes in *nix paths need to be trimmed to not mess up RTL -->
                <div class="font-mono grow bg-surface-50-900-token px-2 py-1 col-span-9 truncate text-end" dir="rtl">{trimLeftChar(tracks[page.page].filenames[0], "/")}</div>
            </div>
        {/if}
        <hr class="!border-t-4 !border-dashed" />
        <InfoSegment header="Integrity" value={tracks[page.page].test_and_copy.integrity} icon={IconDoubleInteger} />
        <InfoSegment header="Integrity (skip zeroes)" value={tracks[page.page].test_and_copy.integrity_skipzero} icon={IconDoubleInteger} />
        
        {#if tracks[page.page].test_and_copy.integrity === 'Match' }
            <ChecksumSegment header="T&C hash" hash={tracks[page.page].test_and_copy.test_hash} icon={IconHashtag} status={tracks[page.page].test_and_copy.integrity} />
        {:else}
            <ChecksumSegment header="Test hash" hash={tracks[page.page].test_and_copy.test_hash} icon={IconHashtag} status={tracks[page.page].test_and_copy.integrity} />
            <ChecksumSegment header="Copy hash" hash={tracks[page.page].test_and_copy.copy_hash} icon={IconHashtag} status={tracks[page.page].test_and_copy.integrity} />
        {/if}

        {#if tracks[page.page].test_and_copy.integrity_skipzero === 'Match' }
            <ChecksumSegment header="T&C hash (skip zeroes)" hash={tracks[page.page].test_and_copy.test_skipzero_hash} icon={IconHashtag} status={tracks[page.page].test_and_copy.integrity_skipzero} />
        {:else}
        <ChecksumSegment header="Test hash (skip zeroes)" hash={tracks[page.page].test_and_copy.test_skipzero_hash} icon={IconHashtag} status={tracks[page.page].test_and_copy.integrity_skipzero} />
        <ChecksumSegment header="Copy hash (skip zeroes)" hash={tracks[page.page].test_and_copy.copy_skipzero_hash} icon={IconHashtag} status={tracks[page.page].test_and_copy.integrity_skipzero} />
        {/if}
    </div>
    {#if Object.keys(tracks[page.page].errors).length > 0}
        <hr class="my-4 !border-t-4 !border-dashed" />
        <div class="flex items-center"><IconWarningAlt /><span class="ml-1 dark:font-light text-sm">Track Errors</span></div>
        <Accordion regionPanel="space-y-2">
            {#each Object.keys(tracks[page.page].errors) as errorType}
                <AccordionItem>
                    <svelte:fragment slot="lead">
                        <IconCheckmarkFilledError />
                    </svelte:fragment>
                    <svelte:fragment slot="summary">
                        <div class="flex justify-between items-center">
                            <span>{errorType}</span>
                            <span class="chip variant-soft-error rounded-full">{tracks[page.page].errors[errorType].count}</span>
                        </div>
                    </svelte:fragment>
                    <svelte:fragment slot="content">
                        {#if tracks[page.page].errors[errorType].ranges.length > 0}
                            {#each tracks[page.page].errors[errorType].ranges as errorRange}
                                <div class="flex justify-between items-center">
                                    <div>
                                        <span class="chip variant-soft-success rounded-full">Start</span>
                                        <span class="text-sm">{errorRange.start}</span>
                                    </div>
                                    <div>
                                        <span class="chip variant-soft-success rounded-full">Length</span>
                                        <span class="text-sm">{errorRange.length}</span>
                                    </div>
                                </div>
                            {/each}
                        {/if}
                    </svelte:fragment>
                </AccordionItem>
            {/each}
        </Accordion>
    {/if}
</div>