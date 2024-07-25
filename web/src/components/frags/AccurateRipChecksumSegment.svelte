<script lang="ts">
	import type { AccurateRipOffset } from "$lib/types/AccurateRipOffset";
    import IconHashtag from '~icons/carbon/hashtag';

    export let header: string;
    export let sign: string;
    export let arOffset: AccurateRipOffset | undefined = undefined;
    let offset: number | null = null;

    if (arOffset && arOffset !== "Same") {
        offset = arOffset.Different;
    }
</script>

{#if sign}
    <div class="flex flex-col gap-2">
        <div class="flex items-center">
            <div class="min-w-4">
                <IconHashtag />
            </div>
            <span class="ml-1.5 dark:font-light text-sm">{header}</span>
            {#if offset !== null}
                <span class="ml-2 variant-soft-primary rounded-md text-xs px-1.5 py-0.5 font-semibold">{offset}</span>
            {/if}
        </div>
        <div class="font-mono grow bg-success-900 bg-surface-50-900-token px-2 py-1 truncate">{sign}</div>
    </div>
{/if}