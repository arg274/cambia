<script lang="ts">
	import { Pagination } from 'bits-ui';
	import IconArrowLeft from '~icons/carbon/arrow-left';
	import IconArrowRight from '~icons/carbon/arrow-right';

	interface Props {
		/** Total number of items (one per page). */
		count: number;
		/** Zero-based active page, to match the rest of the app. */
		page: number;
		/** Receives the new zero-based page, for callers that do not bind. */
		onPageChange?: (page: number) => void;
	}

	let { count, page = $bindable(), onPageChange }: Props = $props();

	const buttonClass =
		'px-3 py-1.5 text-sm fill-current disabled:opacity-50 disabled:cursor-not-allowed';
</script>

<!--
	Arrows either side of an "x of n" readout. Bits' pagination is one-based;
	the app tracks pages from zero.
-->
<Pagination.Root
	{count}
	perPage={1}
	page={page + 1}
	onPageChange={(next) => {
		page = next - 1;
		onPageChange?.(page);
	}}
	class="inline-flex flex-row items-center overflow-hidden isolate rounded-(--radius-base) preset-filled"
>
	<Pagination.PrevButton class={buttonClass} aria-label="Previous page">
		<IconArrowLeft class="w-3.5 h-3.5" />
	</Pagination.PrevButton>
	<span class="{buttonClass} pointer-events-none">
		{page + 1}&nbsp;<span class="opacity-50">of {count}</span>
	</span>
	<Pagination.NextButton class={buttonClass} aria-label="Next page">
		<IconArrowRight class="w-3.5 h-3.5" />
	</Pagination.NextButton>
</Pagination.Root>
