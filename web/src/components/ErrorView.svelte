<script lang="ts">
	import type { CambiaError } from "$lib/types/CambiaError";
	import { hexify } from "$lib/utils";

    export let err: CambiaError;
</script>

<div class="px-4 grid h-screen place-items-center">
    <div class="flex flex-col items-center">
        <div class="flex flex-col">
            <span class="text-5xl md:text-8xl mt-8 font-bold">oops.</span>
            <span class="text-xl md:text-2xl mt-2">oops, something went wrong.</span>
            <div class="mt-8">
                {#if err.id}
                    <span class="h-6 chip variant-soft-error rounded-full pointer-events-none">{hexify(err.id)}</span>
                {/if}
                <span class="text-error-700 dark:text-error-400 md:text-sm mt-8 lowercase">{err.message}</span>
            </div>
        </div>
    </div>
</div>