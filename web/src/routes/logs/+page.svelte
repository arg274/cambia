<script>
    import { fade } from 'svelte/transition';
	import MultiLogView from "../../components/MultiLogView.svelte";
	import { fileListStore } from '$lib/LogStore';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import LogAggregate from '../../components/frags/LogAggregate.svelte';

    onMount(() => {
        if (!$fileListStore || ($fileListStore && $fileListStore.length == 0)) {
            goto('/');
        }
    });

</script>

{#if $fileListStore && $fileListStore.length > 1}
    <div class="flex justify-center">
        <div class="flex flex-col px-4 mt-10 w-full xl:w-3/4 2xl:w-1/2 md:max-lg:self-start gap-4">
            <LogAggregate />
            <MultiLogView />
        </div>
    </div>
{/if}