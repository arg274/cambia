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
	import { errorStore, fileListStore, hashIndexLookup, inputChanged, processedCount, responseStore} from '$lib/LogStore';
	import { onMount } from 'svelte';
	import { page } from '$app/state';

	import type { CambiaError } from '$lib/types/CambiaError';
	import { removeRoute } from '$lib/utils';
	import LoadModal from '../components/frags/LoadModal.svelte';

	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();

	// v2 drove this through the modal store; LoadModal is itself a full-screen
	// overlay, so a plain flag is enough.
	let loading = $state(false);

	function onToggleHandler(): void {
		$modeCurrent = !$modeCurrent;
		setModeUserPrefers($modeCurrent);
		setModeCurrent($modeCurrent);
	}

	afterNavigate((params: AfterNavigate) => {
		const isNewPage = params.from?.url.pathname !== params.to?.url.pathname;
		const elemPage = document.querySelector('#page');
		if (isNewPage && elemPage !== null) elemPage.scrollTop = 0;
	});

	onMount(() => {
		processedCount.subscribe(p => {
			if ($fileListStore?.length == 1 && p == 0) {
				loading = true;
			} else if ($fileListStore?.length == 1 && p == 1) {
				loading = false;
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
			} else if ($fileListStore && $fileListStore.length > 1) {
				if (location.pathname !== '/logs') goto(`${removeRoute(location.pathname, page.route.id)}/logs`);
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
	<DropScreen bind:files={$fileListStore} onchange={() => {inputChanged(page.route.id)}} >
		{@render children?.()}
	</DropScreen>
	<footer class="mt-10 rounded-tr-xl bg-surface-100-800 px-4 py-1">
		<div class="flex items-center justify-between">
			<div>
				<CambiaLogo class="w-5 stroke-surface-300 dark:stroke-surface-400 stroke-1" />
			</div>
			<div>
				<a href="https://github.com/arg274/cambia" class="btn-icon hover:preset-tonal" target="_blank"><IconGithub class="icon-lg" /></a>
			</div>
		</div>
	</footer>
</div>
