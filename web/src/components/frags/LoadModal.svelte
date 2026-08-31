<script lang="ts">
	import { Progress } from "@skeletonlabs/skeleton-svelte";
	import CambiaLogo from "../icons/CambiaLogo.svelte";
</script>
<div class="fixed top-0 left-0 right-0 w-screen h-screen bg-surface-100-900">
    <div class="grid h-screen place-items-center">
        <div class="flex flex-col gap-4">
            <CambiaLogo class="stroke-primary-400 w-20" />
            <Progress value={null}>
                <Progress.Track class="h-1 rounded-full bg-surface-200-800">
                    <Progress.Range class="h-full rounded-full bg-primary-500 dark:bg-primary-400" />
                </Progress.Track>
            </Progress>
        </div>
    </div>
</div>
