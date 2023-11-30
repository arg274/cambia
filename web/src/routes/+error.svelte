<script lang="ts">
    import { page } from '$app/stores';
	import DecoGrid from '../components/frags/DecoGrid.svelte';
</script>

<div class="px-4 grid h-screen place-items-center">
    <div class="flex flex-col items-center">
        <DecoGrid color="bg-error-700 dark:bg-error-400" />
        <div class="flex flex-col">
            <span class="text-5xl md:text-8xl mt-8 font-bold">oops.</span>
            <span class="text-xl md:text-2xl mt-2">oops, something went wrong.</span>
            {#if $page.error}
                <div class="mt-8">
                    {#if $page.status}
                        <span class="h-6 chip variant-soft-error rounded-full pointer-events-none">{$page.status}</span>
                    {/if}
                    {#if $page.error.message}
                        <span class="text-error-700 dark:text-error-400 md:text-sm mt-8 lowercase">{$page.error.message}.</span>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</div>