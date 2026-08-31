<script lang="ts">
    interface Props {
        scope: string;
    }

    let { scope }: Props = $props();
</script>

<div class="text-xs py-1 px-2 preset-tonal-primary rounded-md">{scope}</div>