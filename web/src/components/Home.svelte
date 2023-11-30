<script lang="ts">
	import { onMount } from "svelte";
    import IconCambiaOutline from '~icons/cambia/cambia-outline';
	import { runAnimation } from "$lib/utils";
    
    onMount(() => {
        runAnimation("big-logo");
    });
</script>

<div class="flex flex-col md:items-center">
    <IconCambiaOutline class="stroke-tertiary-400 text-8xl md:text-[14rem]" id="big-logo" />
    <div class="flex flex-col">
        <span class="text-5xl md:text-8xl mt-8 tracking-tighter">this is <strong class="gradient-text">cambia.</strong></span>
        <span class="text-xl md:text-2xl mt-2">a tool for assessing compact disc rip logs.</span>
        <span class="text-lg md:text-xl mt-8">drop your files anywhere to get started.</span>
    </div>
</div>