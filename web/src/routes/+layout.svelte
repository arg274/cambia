<script lang='ts'>
	import '../app.postcss';

	import { AppBar, AppShell, initializeStores, setInitialClassState, Toast, modeCurrent, setModeUserPrefers, setModeCurrent } from '@skeletonlabs/skeleton';
	
	import CambiaLogo from '../components/icons/CambiaLogo.svelte';
	import IconWindowBlackSaturation from '~icons/carbon/window-black-saturation';
	import IconGithub from '~icons/carbon/logo-github';
	import { fade } from 'svelte/transition';

	import type { AfterNavigate } from '@sveltejs/kit';
	import { afterNavigate, goto } from '$app/navigation';
	import DropScreen from '../components/frags/DropScreen.svelte';
	import { errorStore, fileListStore, hashIndexLookup, inputChanged, processedCount, responseStore} from '$lib/LogStore';
	import { onMount } from 'svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import type { CambiaError } from '$lib/types/CambiaError';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	
	initializeStores();

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
			if ($fileListStore?.length == 1 && p == 1) {
				switch ($responseStore[0].status) {
					case "processed":
						goto(`/log?id=${hashIndexLookup.keys().next().value}`);
						break;
					case "errored":
						errorStore.set($responseStore[0].content as CambiaError);
						goto("/error")
						break;
					default:
						console.log("Error");
						break;
				}
			} else if ($fileListStore && $fileListStore.length > 1) {
				goto('/logs');
			}
		});

		// FIXME: Unwanted detection in forms and breaks if pasted in high frequency
		document.addEventListener("paste", (ev) => {
			const dt = ev.clipboardData;
			const tmp_dt = new DataTransfer();
			if (dt) {
				if (dt.types.includes("text/plain")) {
					const file = new File([dt.getData("text")], "pasted.log", {type: 'text/plain'});
					tmp_dt.items.add(file);
				} else {
					for (const file of dt.files) {
						if (file.name.endsWith(".log") || file.name.endsWith(".txt")) {
							tmp_dt.items.add(file);
						}
					}
				}
				fileListStore.set(tmp_dt.files);
				inputChanged();
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
						<span>cambia</span>
						<CambiaLogo class="w-5 stroke-black dark:stroke-white stroke-1" />
						<span><strong>LogTools</strong></span>
					</div>
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<button type="button" class="btn-icon bg-initial hover:variant-soft" on:click={onToggleHandler}><IconWindowBlackSaturation class="icon-lg" /></button>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<DropScreen bind:files={$fileListStore} on:change={inputChanged} >
		<slot />
	</DropScreen>
	<svelte:fragment slot="pageFooter">
		<AppBar class="mt-10" background="bg-surface-100-800-token">
			<svelte:fragment slot="lead">
				<CambiaLogo class="w-5 stroke-surface-300 dark:stroke-surface-400 stroke-1" />
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<a href="https://github.com/arg274/cambia" class="btn-icon bg-initial hover:variant-soft" target="_blank"><IconGithub class="icon-lg" /></a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
</AppShell>