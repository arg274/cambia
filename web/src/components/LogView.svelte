<script lang="ts">
	import Paginator from './frags/Paginator.svelte';
	import classNames from 'classnames';
	import IconArrowRight from '~icons/carbon/arrow-right';

	import EvaluationInfo from './EvaluationInfo.svelte';
	import Grade from './Grade.svelte';
	import RipInfo from './RipInfo.svelte';
	import RipInfoQuartet from './RipInfoQuartet.svelte';
	import TocInfo from './TocInfo.svelte';
	import ReleaseInfo from './ReleaseInfo.svelte';
	import ChecksumInfo from './ChecksumInfo.svelte';
	import TrackInfo from './TrackInfo.svelte';
	import type { CambiaResponse } from '$lib/types/CambiaResponse';
	import { onMount } from 'svelte';
	import AccurateRipSummary from './AccurateRipSummary.svelte';

	interface Props {
		res: CambiaResponse;
	}

	let { res }: Props = $props();
	let inputPage = $state(1);
	let inputEl: HTMLInputElement | undefined = $state();

	// TODO: Any way to get this from Tailwind directly?
	let mq = window.matchMedia('(min-width: 768px)');
	let isMd = $state(mq.matches);

	let pageIndex = $state(0);
	const pageCount = $derived(res.parsed.parsed_logs.length);

	function onPageChange() {
		inputPage = pageIndex + 1;
	}

	function pageInputHandler(ev: KeyboardEvent) {
		switch (ev.key) {
			case ',':
			case '.':
			case '-':
			case 'e':
				ev.preventDefault();
				break;
			case 'Enter':
				ev.preventDefault();
				gotoPage();
				break;
		}
	}

	function selectText() {
		inputEl?.select();
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

	onMount(() => {
		const onMediaChange = (e: MediaQueryListEvent) => {
			isMd = e.matches;
		};
		mq.addEventListener('change', onMediaChange);
		return () => mq.removeEventListener('change', onMediaChange);
	});
</script>

{#if res}
	{@const combinedLog = res.parsed.parsed_logs.length > 1 ? true : false}
	{@const parsedLog = res.parsed.parsed_logs[pageIndex]}
	{@const ev = res.evaluation_combined.filter((ec) => ec.evaluator === 'OPS')[0].evaluations[
		pageIndex
	]}
	{#if combinedLog}
		<div class="flex justify-between items-end md:items-center">
			<span class="text-xs uppercase tracking-widest mb-2 md:mb-0">Combined Log</span>
			<div class="flex justify-center md:justify-end md:items-center items-end gap-2 mb-2">
				<Paginator count={pageCount} bind:page={pageIndex} {onPageChange} />
				<div class="flex justify-end items-center hide-scroll-numinput">
					<input
						type="number"
						required
						bind:value={inputPage}
						class="w-12 h-8 border-0 preset-filled py-1.5 text-center text-sm rounded-l-full"
						onkeypress={pageInputHandler}
						onclick={(e) => { e.preventDefault(); selectText(); }}
						bind:this={inputEl}
					/>
					<button
						type="button"
						class="preset-filled h-8 px-2 text-sm inline-flex items-center justify-center rounded-r-full"
						onclick={gotoPage}><IconArrowRight class="w-3.5 h-3.5" /></button
					>
				</div>
			</div>
		</div>
	{/if}

	<div class={classNames('flex flex-col gap-y-4', combinedLog ? 'mt-4' : '')}>
		<ReleaseInfo mbzTocId={parsedLog.toc.mbz.hash} logRelease={parsedLog.release_info} />
		{#if isMd}
			<div class="flex flex-col gap-y-4">
				<div class="flex gap-x-4">
					<div class="flex flex-col w-1/2 gap-4">
						{#key res.evaluation_combined}
							<Grade evaluations={res.evaluation_combined} />
						{/key}
						{#key pageIndex}
							<EvaluationInfo
								logs={res.parsed}
								combinedEvals={res.evaluation_combined}
								selectedLogIdx={pageIndex}
							/>
						{/key}
						<TocInfo toc={parsedLog.toc} />
						<ChecksumInfo checksum={parsedLog.checksum} />
					</div>
					<div class="flex flex-col w-1/2 gap-4">
						<RipInfo {parsedLog} evaluation={ev} />
						<RipInfoQuartet {parsedLog} evaluation={ev} />
					</div>
				</div>
				<hr class="!border-t-4 !border-dashed" />
				<TrackInfo toc={parsedLog.toc.raw} tracks={parsedLog.tracks} />
				<div class="flex gap-x-4">
					<div class="flex flex-col w-1/2 gap-4">
						<AccurateRipSummary tracks={parsedLog.tracks} />
					</div>
					<!-- CTDB panel is WIP; re-add `import CtdbSummary from './CtdbSummary.svelte'`
					     along with this block.
					<div class="flex flex-col w-1/2 gap-4">
						<CtdbSummary />
					</div> -->
				</div>
			</div>
		{:else}
			<div class="flex flex-col gap-4">
				{#key res.evaluation_combined}
					<Grade evaluations={res.evaluation_combined} />
				{/key}
				{#key pageIndex}
					<EvaluationInfo
						logs={res.parsed}
						combinedEvals={res.evaluation_combined}
						selectedLogIdx={pageIndex}
					/>
				{/key}
				<RipInfo {parsedLog} evaluation={ev} />
				<RipInfoQuartet {parsedLog} evaluation={ev} />
				<ChecksumInfo checksum={parsedLog.checksum} />
				<TocInfo toc={parsedLog.toc} />
				<TrackInfo toc={parsedLog.toc.raw} tracks={parsedLog.tracks} />
				<AccurateRipSummary tracks={parsedLog.tracks} />
			</div>
		{/if}
	</div>
{/if}
