<script lang="ts">
	import anime from 'animejs';

	import { getCssColorHex, toCardId } from '$lib/utils';
	import Home from '../components/Home.svelte';

	function onClickHandler(e: MouseEvent) {
		if (e.target instanceof HTMLLIElement) {
			let rawHeader = (e.target as HTMLLIElement).textContent;
			if (rawHeader) {
				let cardId = `#${toCardId(rawHeader)}`;
				anime({
					targets: cardId,
					borderColor: getCssColorHex('color-primary-400'),
					direction: 'alternate',
					scale: 1.01,
					duration: 500,
					easing: 'easeOutCubic',
				});
			}
		}
	}
</script>

<div class="px-4 grid h-full place-items-center">
	<Home />
</div>