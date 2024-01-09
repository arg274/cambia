<script lang="ts">
	import { onMount } from "svelte";
    import IconCambiaOutline from '~icons/cambia/cambia-outline';
	import { runAnimation } from "$lib/utils";
	import { fileListStore, inputChanged } from "$lib/LogStore";
    
    onMount(() => {
        runAnimation("big-logo");
    });
</script>

<div class="flex flex-col md:items-center">
    <IconCambiaOutline class="stroke-tertiary-400 text-8xl md:text-[14rem]" id="big-logo" />
    <div class="flex flex-col">
        <span class="text-5xl md:text-8xl mt-8 tracking-tighter">this is <strong class="gradient-text">cambia.</strong></span>
        <span class="text-xl md:text-2xl mt-2">a tool for assessing compact disc rip logs.</span>
        <div class="mt-8 items-center text-lg md:text-xl">
            <label for="cursorFiles"><span class="underline decoration-primary-400 decoration-2 md:decoration-4">click here</span>&nbsp;/ paste / drop your files anywhere to get started.</label>
            <input id="cursorFiles" class="hidden" type="file" multiple={true} accept=".log, .txt" bind:files={$fileListStore} on:change={inputChanged} />
        </div>
    </div>
</div>