<script lang='ts'>
	import '../app.postcss';

	import { AppBar, AppShell, initializeStores, setInitialClassState, Toast, modeCurrent, setModeUserPrefers, setModeCurrent } from '@skeletonlabs/skeleton';
	
	import IconCambiaOutline from '~icons/cambia/cambia-outline';
	import IconWindowBlackSaturation from '~icons/carbon/window-black-saturation';
	import { fade } from 'svelte/transition';

	import type { AfterNavigate } from '@sveltejs/kit';
	import { afterNavigate, goto } from '$app/navigation';
	import DropScreen from '../components/frags/DropScreen.svelte';
	import { fileListStore, hashIndexLookup, initialiseResponseStore, processedCount} from '$lib/LogStore';
	import { getRipInfoJsonMulti } from '$lib/api/CambiaApi';
	import { onMount } from 'svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	
	initializeStores();

	let files: FileList | undefined;

	function inputChanged() {
		fileListStore.set(files);
		initialiseResponseStore(files);
		getRipInfoJsonMulti(files);
    }

	function onToggleHandler(): void {
		$modeCurrent = !$modeCurrent;
		setModeUserPrefers($modeCurrent);
		setModeCurrent($modeCurrent);
	}

	afterNavigate((params: AfterNavigate) => {
		const isNewPage = params.from?.url.pathname !== params.to?.url.pathname;
		const elemPage = document.querySelector('#page');
		if (isNewPage && elemPage !== null) {
			elemPage.scrollTop = 0;
    }});

	onMount(() => {
		processedCount.subscribe(p => {
			if (files?.length == 1 && p == 1) {
				goto(`/log?id=${hashIndexLookup.keys().next().value}`);
			} else if (files && files.length > 1) {
				goto('/logs');
			}
		});
	});
</script>

<svelte:head>
	{@html `<script>(${setInitialClassState.toString()})();</script>`}
</svelte:head>

<Toast rounded="rounded-none" transitionIn={fade} transitionOut={fade} transitionInParams={{duration: 100}} transitionOutParams={{duration: 100}} />
<AppShell slotPageHeader="sticky top-0 z-50 backdrop-blur-xl bg-opacity-10" regionPage="scroll-smooth" scrollbarGutter="stable">
	<svelte:fragment slot="pageHeader">
		<AppBar padding="px-4 py-1" background="rounded-br-xl bg-primary-400/10">
			<svelte:fragment slot="lead">
				<a href="/">
					<div class="flex gap-x-2 items-center">
						<span>cambia</span> <IconCambiaOutline class="stroke-black dark:stroke-white stroke-2" /> <span><strong>LogTools</strong></span>
					</div>
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<button type="button" class="btn-icon bg-initial hover:variant-soft" on:click={onToggleHandler}><IconWindowBlackSaturation class="icon-lg" /></button>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<DropScreen bind:files on:change={inputChanged} >
		<slot />
	</DropScreen>
	<!-- <div class="mt-10"></div> -->
</AppShell>