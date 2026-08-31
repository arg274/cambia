<script lang="ts">
    import { responseStore } from "$lib/LogStore";
    import { createTable, Subscribe, Render, createRender } from "@humanspeak/svelte-headless-table";
    import { addPagination } from '@humanspeak/svelte-headless-table/plugins';
    import Paginator from "./frags/Paginator.svelte";
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

    // The table plugin owns the page index; mirror it for the paginator.
    let page = $derived($pageIndex);

    function onPageChange(nextPage: number) {
        pageIndex.update(() => nextPage);
    }

    function getColumnSize(cellId: string): string {
        switch (cellId) {
            case "Lead":
                return "w-7/12";
            case "Score":
                return "w-2/12 sm:w-1/12";
            case "MBZ DiscID":
                return "w-3/12 sm:w-4/12";
            default:
                return "";
        }
    }
</script>

<div class="flex self-end items-center">
    <Paginator count={$pageCount} {page} onPageChange={(next) => onPageChange(next)} />
</div>

<table class="w-full table-fixed" {...$tableAttrs}>
    <tbody {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
            <Subscribe rowAttrs={row.attrs()}>
                {#snippet children({ rowAttrs }: { rowAttrs: Record<string, unknown> })}
                    <tr class="bg-surface-100-900" {...rowAttrs}>
                        {#each row.cells as cell (cell.id)}
                            <Subscribe attrs={cell.attrs()}>
                                {#snippet children({ attrs }: { attrs: Record<string, unknown> })}
                                    <td class="py-4 px-2 {getColumnSize(cell.id)}" {...attrs}>
                                        <Render of={cell.render()} />
                                    </td>
                                {/snippet}
                            </Subscribe>
                        {/each}
                    </tr>
                    <div class="h-2"></div>
                {/snippet}
            </Subscribe>
        {/each}
    </tbody>
</table>
