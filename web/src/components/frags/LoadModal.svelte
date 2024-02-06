<script>
	import { ProgressBar } from "@skeletonlabs/skeleton";
	import CambiaLogo from "../icons/CambiaLogo.svelte";

</script>
<div class="fixed top-0 left-0 right-0 w-screen h-screen bg-surface-100-800-token">
    <div class="grid h-screen place-items-center">
        <div class="flex flex-col gap-4">
            <CambiaLogo class="stroke-primary-400 w-20" />
            <ProgressBar value={undefined} height="h-1" rounded="rounded-full" meter="bg-primary-500 dark:bg-primary-400" />
        </div>
    </div>
</div>