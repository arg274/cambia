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
    
    export let tracks: TrackEntry[];
    export let selectedTrack: number;

    let page = {
        page: 0,
        limit: 1,
        size: tracks.length,
        amounts: [1]
    } as PaginationSettings;
    $: paginatedTracks = tracks.slice(
        page.page * page.limit,
        page.page * page.limit + page.limit
    );

    let inputPage: number = 1;
    let inputEl: HTMLInputElement;

    function onPageChange() {
        inputPage = page.page + 1;
    }

    function enterHandler(ev: KeyboardEvent) {
        if (ev.key == "Enter") {
            ev.preventDefault();
            gotoPage();
        }
    }

    function gotoPage() {
        // FIXME: Needs sanitisation
        if (inputPage > page.size) {
            return;
        }
        page = {
            page: inputPage - 1,
            limit: 1,
            size: tracks.length,
            amounts: [1]
        }
    }

    function selectText(e: MouseEvent) {
        inputEl.select();
    }

    $: {
        inputPage = selectedTrack;
        gotoPage();
    }
</script>
<div>
    <div class="flex justify-center md:justify-end md:items-center items-end gap-2 mb-2">
        <Paginator bind:settings={page} on:page={onPageChange}></Paginator>
        <div class="flex justify-end items-center">
            <input bind:value={inputPage} class="w-12 variant-filled py-1.5 text-center text-sm rounded-l-full" on:keypress={enterHandler} on:click|preventDefault={selectText} bind:this={inputEl} />
            <!-- FIXME: Looks weird while resizing in single column -->
            <button type="button" class="variant-filled py-1.5 px-2 rounded-r-full" on:click={gotoPage}><IconArrowRight /></button>
        </div>
    </div>
    {#each paginatedTracks as singleTrack}
        <div class="flex flex-col gap-4">
            <h6>Track {page.page + 1}</h6>
            <div class="flex flex-col gap-4">
                <InfoSegment icon={IconSplitScreen} header="Track splitting" value={singleTrack.is_range ? "Range" : "Split"} />
                <InfoSegment icon={IconIncompleteCancel} header="Aborted" value={singleTrack.aborted} />
                <InfoSegment icon={IconSidePanelOpenFilled} header="Extraction speed" value={`${singleTrack.extraction_speed?.toFixed(1)}x`} />
                <InfoSegment icon={IconMountain} header="Peak level" value={singleTrack.peak_level?.toFixed(3)} />
                <InfoSegment icon={IconMicrophone} header="Gain" value={singleTrack.gain} />
                <InfoSegment icon={IconExpandCategories} header="Pregap length" value={singleTrack.pregap_length ? `${parseFloat(singleTrack.pregap_length).toFixed(2)} sec` : null} />
                <InfoSegment icon={IconTransmissionLte} header="Pre-emphasis" value={singleTrack.preemphasis} />
            </div>
            {#if singleTrack.filename}
                <div class="flex flex-col gap-2">   
                    <div class="flex items-center"><IconDocumentBlank class="icon-sm" /><span class="ml-1 dark:font-light text-sm">Filename</span></div>
                    <!-- FIXME: Weird slash bug for *nix paths -->
                    <div class="font-mono grow bg-surface-50-900-token px-2 py-1 col-span-9 truncate" dir="rtl">{singleTrack.filename}</div>
                </div>
            {/if}
            <hr class="!border-t-4 !border-dashed" />
            <InfoSegment header="Integrity" value={singleTrack.test_and_copy.integrity} icon={IconDoubleInteger} />
            <InfoSegment header="Integrity (skip zeroes)" value={singleTrack.test_and_copy.integrity_skipzero} icon={IconDoubleInteger} />
            
            {#if singleTrack.test_and_copy.integrity === 'Match' }
                <ChecksumSegment header="T&C hash" hash={singleTrack.test_and_copy.test_hash} icon={IconHashtag} status={singleTrack.test_and_copy.integrity} />
            {:else}
                <ChecksumSegment header="Test hash" hash={singleTrack.test_and_copy.test_hash} icon={IconHashtag} status={singleTrack.test_and_copy.integrity} />
                <ChecksumSegment header="Copy hash" hash={singleTrack.test_and_copy.copy_hash} icon={IconHashtag} status={singleTrack.test_and_copy.integrity} />
            {/if}

            {#if singleTrack.test_and_copy.integrity_skipzero === 'Match' }
                <ChecksumSegment header="T&C hash (skip zeroes)" hash={singleTrack.test_and_copy.test_skipzero_hash} icon={IconHashtag} status={singleTrack.test_and_copy.integrity_skipzero} />
            {:else}
            <ChecksumSegment header="Test hash (skip zeroes)" hash={singleTrack.test_and_copy.test_skipzero_hash} icon={IconHashtag} status={singleTrack.test_and_copy.integrity_skipzero} />
            <ChecksumSegment header="Copy hash (skip zeroes)" hash={singleTrack.test_and_copy.copy_skipzero_hash} icon={IconHashtag} status={singleTrack.test_and_copy.integrity_skipzero} />
            {/if}
        </div>
        {#if Object.keys(singleTrack.errors).length > 0}
            <hr class="my-4 !border-t-4 !border-dashed" />
            <div class="flex items-center"><IconWarningAlt /><span class="ml-1 dark:font-light text-sm">Track Errors</span></div>
            <Accordion regionPanel="space-y-2">
                {#each Object.keys(singleTrack.errors) as errorType}
                    <AccordionItem>
                        <svelte:fragment slot="lead">
                            <IconCheckmarkFilledError />
                        </svelte:fragment>
                        <svelte:fragment slot="summary">
                            <div class="flex justify-between items-center">
                                <span>{errorType}</span>
                                <span class="chip variant-soft-error rounded-full">{singleTrack.errors[errorType].count}</span>
                            </div>
                        </svelte:fragment>
                        <svelte:fragment slot="content">
                            {#if singleTrack.errors[errorType].ranges.length > 0}
                                {#each singleTrack.errors[errorType].ranges as errorRange}
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
    {/each}
</div>