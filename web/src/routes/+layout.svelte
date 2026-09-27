<script lang='ts'>
	import '../app.css';

	// Skeleton v5 dropped the lightswitch utilities; this repo already carries a
	// copy of the v2 implementation, so the toggle keeps behaving as before.
	import { modeCurrent, setModeUserPrefers, setModeCurrent } from '$lib/lightswitch';

	import CambiaLogo from '../components/icons/CambiaLogo.svelte';
	import IconHelp from '~icons/carbon/help';
	import IconWindowBlackSaturation from '~icons/carbon/window-black-saturation';
	import IconGithub from '~icons/carbon/logo-github';

	import type { AfterNavigate } from '@sveltejs/kit';
	import { afterNavigate, goto } from '$app/navigation';
	import DropScreen from '../components/frags/DropScreen.svelte';
	import { errorStore, fileListStore, hashIndexLookup, clearFileList, inputChanged, processedCount, processing, responseStore} from '$lib/LogStore';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { page } from '$app/state';

	import type { CambiaError } from '$lib/types/CambiaError';
	import { removeRoute } from '$lib/utils';
	import LoadModal from '../components/frags/LoadModal.svelte';

	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();

	// LoadModal is itself a full-screen overlay, so a flag is enough. It tracks
	// an upload actually being in flight rather than the file list, which
	// otherwise left the overlay up whenever a stale single file was around.
	const loading = $derived($processing && $fileListStore?.length === 1);

	function onToggleHandler(): void {
		$modeCurrent = !$modeCurrent;
		setModeUserPrefers($modeCurrent);
		setModeCurrent($modeCurrent);
	}

	afterNavigate((params: AfterNavigate) => {
		const isNewPage = params.from?.url.pathname !== params.to?.url.pathname;
		const elemPage = document.querySelector('#page');
		if (isNewPage && elemPage !== null) elemPage.scrollTop = 0;

		// Drop the selection on returning home. Not on the result routes: /logs
		// reads the list as its "is there a batch" signal, so clearing it there
		// would blank the page and break going back to it from a single log.
		if (params.to?.route.id === '/') clearFileList();
	});

	onMount(() => {
		const unsubscribe = processedCount.subscribe(p => {
			// Only meaningful while an upload is running; subscribing replays the
			// current value, and the store survives client-side navigation.
			if (!get(processing) || $fileListStore?.length !== 1 || p !== 1) return;

			processing.set(false);
			switch ($responseStore[0].status) {
				case "processed":
					goto(`${removeRoute(location.pathname, page.route.id)}/log?id=${hashIndexLookup.keys().next().value}`);
					break;
				case "errored":
					errorStore.set($responseStore[0].content as CambiaError);
					goto(`${removeRoute(location.pathname, page.route.id)}/error`)
					break;
				default:
					console.log("Error");
					break;
			}
		});

		// FIXME: Breaks if pasted in high frequency
		document.addEventListener("paste", (ev) => {
			const dt = ev.clipboardData;
			const tmp_dt = new DataTransfer();
			if (dt && !(ev.target instanceof HTMLInputElement || ev.target instanceof HTMLTextAreaElement)) {
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
				inputChanged(page.route.id);
			}
		});

		return unsubscribe;
	});
</script>

{#if loading}
	<LoadModal />
{/if}

<!-- Skeleton v5 removed AppShell; this is the same three-region layout by hand. -->
<div id="page" class="w-full h-full grid grid-rows-[auto_1fr_auto] overflow-y-auto overflow-x-hidden scroll-smooth" style="scrollbar-gutter: stable;">
	<div class="sticky top-0 z-50 backdrop-blur-xl">
		<header class="bg-primary-400/10 px-4 py-1">
			<div class="flex items-center justify-between">
				<div>
					<a href="{removeRoute(page.url.pathname, page.route.id)}/">
						<div class="flex gap-x-2 items-center">
							<span>cambia</span>
							<CambiaLogo class="w-5 stroke-black dark:stroke-white stroke-1" />
							<span><strong>LogTools</strong></span>
						</div>
					</a>
				</div>
				<div>
					<div class="flex gap-x-0">
						<a type="button" class="btn-icon hover:preset-tonal" href="{removeRoute(page.url.pathname, page.route.id)}/help"><IconHelp class="icon-lg" /></a>
						<button type="button" class="btn-icon hover:preset-tonal" onclick={onToggleHandler}><IconWindowBlackSaturation class="icon-lg" /></button>
					</div>
				</div>
			</div>
		</header>
	</div>
	<!--
		The middle row needs an element that is always in flow. DropScreen hides
		its content while a drag is over the page, and without this the footer
		auto-placed itself into the 1fr row and jumped up behind the overlay.
		AppShell used to provide it as <main>, which also restores the landmark.
	-->
	<main>
		<DropScreen bind:files={$fileListStore} onchange={() => {inputChanged(page.route.id)}} >
			{@render children?.()}
		</DropScreen>
	</main>
	<footer class="mt-10 rounded-tr-xl bg-surface-100-800 px-4 py-1">
		<div class="flex items-center justify-between">
			<div>
				<CambiaLogo class="w-5 stroke-surface-300 dark:stroke-surface-400 stroke-1" />
			</div>
			<div>
				<a href="https://github.com/rokkhonorg/cambia" class="btn-icon hover:preset-tonal" target="_blank"><IconGithub class="icon-lg" /></a>
			</div>
		</div>
	</footer>
</div>
