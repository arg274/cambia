<script lang="ts">
	import { Progress } from "bits-ui";
	import CambiaLogo from "../icons/CambiaLogo.svelte";
</script>
<div class="fixed top-0 left-0 right-0 w-screen h-screen bg-surface-100-900">
    <div class="grid h-screen place-items-center">
        <div class="flex flex-col gap-4">
            <CambiaLogo class="stroke-primary-400 w-20" />
            <!-- value={null} renders the indeterminate state -->
            <Progress.Root
                value={null}
                class="relative h-1 w-full overflow-hidden rounded-full bg-surface-200-800"
            >
                <div class="h-full w-2/5 rounded-full bg-primary-500 dark:bg-primary-400 animate-indeterminate"></div>
            </Progress.Root>
        </div>
    </div>
</div>
