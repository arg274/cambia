<script lang="ts">
	import type { Quartet } from "$lib/types/Quartet";
	import { getInfoOverviewPopoverText } from "$lib/utils";
    import { popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from "@skeletonlabs/skeleton";

    export let gradeMap: Map<string, string>[];
    export let index: number;
    export let gradeKey: string;
    export let actualValue: Quartet;
    export let miniName: string;
    export let hideLabel: boolean;

    let popupHover: PopupSettings = {
        event: 'hover',
        target: miniName,
        placement: 'top'
    };

    // FIXME: This is a temp hack
    function opsMap(val: string | undefined): string {
        switch (val) {
            case undefined:
                return "Good";
            case "-1" || "0":
                return "NotIdeal";
            default:
                return "Bad";
        }
    }

    function getColorWrapperDiv(idx: number, key: string): string {
        const grade = opsMap(gradeMap[idx].get(key));
        const base = 'border-t-2 opacity-60 hover:opacity-100 bg-gradient-to-b to-transparent ';
        const good = 'border-success-600 from-success-700/40';
        const notIdeal = 'border-warning-600 from-warning-700/40';
        const bad = 'border-error-600 dark:border-error-400 from-error-700/40';

        switch (grade) {
            case 'Good':
                return base + good;
            case 'NotIdeal':
                return base + notIdeal;
            case 'Bad':
                return base + bad;
            default:
                return base + good;
        }
    }

    function getHeight(quartet: Quartet): string {
        switch (quartet) {
            case 'True':
                return '8';
            case 'False':
                return '6';
            case 'Unknown':
                return '4';
            case 'Unsupported':
                return '2';
            default:
                return '0';
        }
    }

    function getColorWrapperText(idx: number, key: string): string {
        const grade = opsMap(gradeMap[idx].get(key));
        switch (grade) {
            case 'Good':
                return 'text-success-700 dark:text-success-600';
            case 'NotIdeal':
                return 'text-warning-600';
            case 'Bad':
                return 'text-error-600 dark:text-error-500';
            default:
                return '';
        }
    }
</script>

<div class="relative flex-auto flex place-content-center {getColorWrapperDiv(index, gradeKey)} h-{getHeight(actualValue)} [&>*]:pointer-events-none" use:popup={popupHover}>
    {#if !hideLabel}
        <div class="hidden sm:block text-xs absolute -bottom-4 {getColorWrapperText(index, gradeKey)}">{miniName}</div>
    {/if}
</div>

<!-- https://github.com/skeletonlabs/skeleton/issues/1019 -->
<div class="card p-2 variant-glass-surface z-50" data-popup={miniName}>
	<p class="text-xs">{getInfoOverviewPopoverText(miniName)}</p>
</div>