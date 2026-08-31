<script lang="ts">
	import { Pagination } from '@skeletonlabs/skeleton-svelte';
	import IconChevronLeft from '~icons/carbon/chevron-left';
	import IconChevronRight from '~icons/carbon/chevron-right';

	interface Props {
		/** Total number of items (one per page). */
		count: number;
		/** Zero-based active page, to match the rest of the app. */
		page: number;
		/** Receives the new zero-based page, for callers that do not bind. */
		onPageChange?: (page: number) => void;
	}

	let { count, page = $bindable(), onPageChange }: Props = $props();

	const triggerClass = 'btn-icon hover:preset-tonal';
	const itemClass =
		'btn-icon hover:preset-tonal data-[selected]:preset-filled-primary-500 cursor-pointer';
</script>

<!-- Skeleton's Pagination is one-based; the app tracks pages from zero. -->
<Pagination
	{count}
	pageSize={1}
	page={page + 1}
	onPageChange={(details) => {
		page = details.page - 1;
		onPageChange?.(page);
	}}
	class="flex items-center gap-1"
>
	<Pagination.PrevTrigger class={triggerClass}><IconChevronLeft /></Pagination.PrevTrigger>
	<Pagination.Context>
		{#snippet children(api)}
			{#each api().pages as pageItem, index (index)}
				{#if pageItem.type === 'page'}
					<Pagination.Item {...pageItem} class={itemClass}>{pageItem.value}</Pagination.Item>
				{:else}
					<Pagination.Ellipsis {index} class="px-1">&hellip;</Pagination.Ellipsis>
				{/if}
			{/each}
		{/snippet}
	</Pagination.Context>
	<Pagination.NextTrigger class={triggerClass}><IconChevronRight /></Pagination.NextTrigger>
</Pagination>
