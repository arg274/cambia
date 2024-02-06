<script lang="ts">
    import IconWindows from '~icons/simple-icons/windows';
    import IconMac from '~icons/file-icons/finder';
    import IconLinux from '~icons/mingcute/linux-fill';
    import IconArrowUpRight from '~icons/carbon/arrow-up-right';
    import IconUnknown from '~icons/carbon/unknown';
	import Orpheus from '../../components/icons/Orpheus.svelte';
	import Card from '../../components/frags/Card.svelte';

    import type { PageData } from './$types';
	import type { ComponentType } from 'svelte';

    export let data: PageData;

    function getPlatformLogo(platform: string): ComponentType {
        switch (platform.toLowerCase()) {
            case "windows":
                return IconWindows;
            case "mac":
                return IconMac;
            case "linux":
                return IconLinux;
            default:
                return IconUnknown;
        }
    }

    function getEvaluatorLogo(evaluator: string): ComponentType {
        switch (evaluator.toLowerCase()) {
            case "orpheus":
                return Orpheus;
            default:
                return IconUnknown;
        }
    }
</script>

<div class="mt-10 px-4 flex justify-center">
    <div class="w-full xl:w-3/4 2xl:w-1/2 md:max-lg:self-start flex flex-col gap-4 help-page">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Card header="Supported rippers">
                <div class="ml-1 text-sm flex flex-col gap-y-1">
                    {#each data.rippers as ripper}
                        <div class="flex justify-between">
                            <div>
                                <span class="py-0.5">{ripper.name}</span>
                                {#if ripper.experimental}
                                    <span class="ml-1 px-2 py-0.5 variant-soft-primary rounded-full text-xs uppercase">Experimental</span>
                                {/if}
                            </div>
                            <div class="flex gap-4 items-center">
                                <svelte:component this={getPlatformLogo(ripper.platform)} class="icon-sm" />
                                <a href={ripper.link} class="bg-initial hover:variant-soft" target="_blank">
                                    <IconArrowUpRight class="icon-xs" />
                                </a>
                            </div>
                        </div>
                    {/each}
                </div>
            </Card>
            <Card header="Supported rippers">
                <div class="ml-1 flex flex-col gap-y-2">
                    <div class="text-sm flex flex-col gap-y-1">
                        {#each data.evaluators as evaluator}
                            <div class="flex justify-between">
                                <span>{evaluator.name}</span>
                                <div class="flex gap-4 items-center">
                                    <div class="w-5">
                                        <svelte:component this={getEvaluatorLogo(evaluator.name)} class="icon-sm" />
                                    </div>
                                    <a href={evaluator.link} class="bg-initial hover:variant-soft" target="_blank">
                                        <IconArrowUpRight class="icon-xs" />
                                    </a>
                                </div>
                            </div> 
                        {/each}
                    </div>
                </div>
            </Card>
        </div>
        <!-- <h3 class="font-bold text-2xl text-primary-800 dark:text-primary-200">Why?</h3> -->
    </div>
</div>