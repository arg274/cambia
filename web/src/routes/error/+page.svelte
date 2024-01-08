<script lang="ts">
    import { errorStore } from '$lib/LogStore';
	import ErrorView from '../../components/ErrorView.svelte';
</script>

<!-- TODO: Shows dbpa when valid log is dropped from error -->
{#if $errorStore}
    <ErrorView err={$errorStore} />
{/if}