<script lang="ts">
    import anime from 'animejs';
	import classNames from 'classnames';
	import { onMount } from 'svelte';

    let row = 10;
    let column = 10;
    export let color = "bg-primary-400-500-token"

    onMount(() => {
        anime({
            targets: '#deco-grid > .deco-grid-el',
            scale: [
                {value: .1, easing: 'easeOutSine', duration: 500},
                {value: 1, easing: 'easeInOutQuad', duration: 1200}
            ],
            rotate: [
                {value: 90, easing: 'easeOutSine', duration: 500},
            ],
            delay: anime.stagger(300, {grid: [row, column], from: 'center', direction: 'reverse'}),
            autoplay: true,
            loop: true,
        });
    });
</script>

<div class="grid grid-cols-{column} gap-2" id="deco-grid">
    {#each Array(row) as _}
        {#each Array(column) as _}
            <div class={classNames("w-4 h-4 deco-grid-el", color)}></div>
        {/each}
    {/each}
</div>