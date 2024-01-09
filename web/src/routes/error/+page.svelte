<script lang="ts">
    import { errorStore } from '$lib/LogStore';
	import ErrorView from '../../components/ErrorView.svelte';
</script>

<!-- TODO: Shows dbpa when valid log is dropped from error -->
{#if $errorStore}
    <div class="px-4 grid h-full place-items-center">
        <ErrorView err={$errorStore} />
    </div>
{/if}