<script>
    import { fade } from 'svelte/transition';
	import MultiLogView from "../../components/MultiLogView.svelte";
	import { fileListStore } from '$lib/LogStore';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

    const files = $fileListStore;

    onMount(() => {
        if (!files || (files && files.length == 0)) {
            goto('/');
        }
    });

</script>

{#if files && files.length > 1}
    <div class="flex flex-col px-4 mt-10 items-center" transition:fade={{duration: 150}}>
        <MultiLogView />
    </div>
{/if}