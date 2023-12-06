<script lang="ts">
    import IconUnknown from '~icons/carbon/unknown';
    
    export let header: string = '';
    export let value: string | number | boolean | null | undefined;

    export let icon: ConstructorOfATypedSvelteComponent = IconUnknown;

</script>

{#if value && !value.toString().toLocaleLowerCase().startsWith("null") && !value.toString().toLocaleLowerCase().startsWith("undefined")}
    <div class="flex flex-col gap-0.5">
        <div class="flex items-center">
            <svelte:component this={icon} class="icon-sm" /> <h4 class="ml-1 dark:font-light text-sm">{header}</h4>
        </div>
        <span class="text-xl font-bold">{value}</span>
    </div>
{/if}