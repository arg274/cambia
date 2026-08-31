<script lang="ts">
    interface Props {
        explanation: string;
        children?: import('svelte').Snippet;
    }

    let { explanation, children }: Props = $props();
</script>

<span title={explanation} class="underline decoration-dotted underline-offset-2">{@render children?.()}</span>