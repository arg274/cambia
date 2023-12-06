<script lang="ts">
    import { responseStore } from "$lib/LogStore";
    import { createTable, Subscribe, Render, createRender } from "svelte-headless-table";
    import { addPagination } from 'svelte-headless-table/plugins';
    import { Paginator, type PaginationSettings } from '@skeletonlabs/skeleton';
	import DtDiscId from "./frags/datatable/DtDiscId.svelte";
	import DtScore from "./frags/datatable/DtScore.svelte";
	import DtLead from "./frags/datatable/DtLead.svelte";

    const table = createTable(responseStore, {page: addPagination()});

    const columns = table.createColumns([
        table.column({
            header: 'Lead',
            accessor: (res) => res,
            cell: (val) => createRender(DtLead, {res: val.value}),
        }),
        table.column({
            header: 'Score',
            accessor: (res) => res,
            cell: (val) => createRender(DtScore, {res: val.value}),
        }),
        table.column({
            header: 'MBZ DiscID',
            accessor: (res) => res,
            cell: (val) => createRender(DtDiscId, {res: val.value}),
        }),
    ]);

    const {
        headerRows,
        pageRows,
        tableAttrs,
        tableBodyAttrs,
        pluginStates
    } = table.createViewModel(columns);

    // FIXME: pageIndex isn't memorised
    const {
        pageIndex,
        pageCount,
        pageSize,
        hasNextPage,
        hasPreviousPage
    } = pluginStates.page;

    $: page = {
        page: $pageIndex,
        limit: 1,
        size: $pageCount,
        amounts: [1]
    } as PaginationSettings;

    function onPageChange(e: CustomEvent) {
        pageIndex.update(_ => e.detail as number);
    }
</script>

<div class="flex self-end items-center">
    <Paginator bind:settings={page} on:page={onPageChange}></Paginator>
</div>

<table class="w-full" {...$tableAttrs}>
    <tbody {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
            <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
                <tr class="bg-surface-100-800-token" {...rowAttrs}>
                    {#each row.cells as cell (cell.id)}
                        <Subscribe attrs={cell.attrs()} let:attrs>
                            <td class="py-4 px-2" {...attrs}>
                                <Render of={cell.render()} />
                            </td>
                        </Subscribe>
                    {/each}
                </tr>
                <div class="h-2"></div>
            </Subscribe>
        {/each}
    </tbody>
</table>