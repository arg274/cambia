<script lang="ts">
	import { Pagination } from '@skeletonlabs/skeleton-svelte';
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
	Mirrors Skeleton v2's Paginator, which showed a pair of arrows either side of
	an "x-y of n" readout rather than a button per page. Skeleton's Pagination is
	one-based; the app tracks pages from zero.
-->
<Pagination
	{count}
	pageSize={1}
	page={page + 1}
	onPageChange={(details) => {
		page = details.page - 1;
		onPageChange?.(page);
	}}
	class="inline-flex flex-row items-center overflow-hidden isolate rounded-(--radius-base) preset-filled"
>
	<Pagination.PrevTrigger class={buttonClass} aria-label="Previous page">
		<IconArrowLeft class="w-3.5 h-3.5" />
	</Pagination.PrevTrigger>
	<button type="button" class="{buttonClass} pointer-events-none">
		{page + 1}&nbsp;<span class="opacity-50">of {count}</span>
	</button>
	<Pagination.NextTrigger class={buttonClass} aria-label="Next page">
		<IconArrowRight class="w-3.5 h-3.5" />
	</Pagination.NextTrigger>
</Pagination>
