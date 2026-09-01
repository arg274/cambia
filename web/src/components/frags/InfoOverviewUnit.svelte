<script lang="ts">
	import type { Quartet } from "$lib/types/Quartet";
	import { getInfoOverviewPopoverText } from "$lib/utils";
    import { Tooltip } from '@skeletonlabs/skeleton-svelte';

    interface Props {
        gradeMap: Map<string, string>[];
        index: number;
        gradeKey: string;
        actualValue: Quartet;
        miniName: string;
        hideLabel: boolean;
    }

    let {
        gradeMap,
        index,
        gradeKey,
        actualValue,
        miniName,
        hideLabel
    }: Props = $props();


    // FIXME: This is a temp hack
    function opsMap(val: string | undefined): string {
        switch (val) {
            case undefined:
                return "Good";
            case "-1":
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

<Tooltip openDelay={100} closeDelay={100} positioning={{ placement: 'top' }}>
    <Tooltip.Trigger
        class="relative flex-auto flex place-content-center {getColorWrapperDiv(index, gradeKey)} h-{getHeight(actualValue)} *:pointer-events-none"
    >
        {#if !hideLabel}
            <div class="hidden sm:block text-xs absolute -bottom-4 {getColorWrapperText(index, gradeKey)}">{miniName}</div>
        {/if}
    </Tooltip.Trigger>
    <Tooltip.Positioner>
        <Tooltip.Content class="card p-2 preset-tonal-surface z-50">
            <p class="text-xs">{getInfoOverviewPopoverText(miniName)}</p>
        </Tooltip.Content>
    </Tooltip.Positioner>
</Tooltip>
