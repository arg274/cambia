<script lang="ts">
	import { ListBox, ListBoxItem, popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import classNames from 'classnames';
	import { onMount } from "svelte";
	
	import IconChevronDown from '~icons/carbon/chevron-down';

	export let addListClass: string = "";
	export let addTriggerClass: string = "";
	export let textClass: string = "";
	export let items: string[] = [];
	export let name: string = "";

    let comboboxValue: string;

    let popupCombobox: PopupSettings = {
        event: 'focus-click',
        target: 'combobox',
        placement: 'bottom',
        // Close the popup when the item is clicked
        closeQuery: '.listbox-item'
    };

	onMount(async () => {
		if (items.length !== 0) {
			comboboxValue = items[0];
		}
	})
</script>

<button class={classNames("btn bg-surface-200-700-token pr-2", addTriggerClass)} use:popup={popupCombobox}>
	<span class={textClass}>{comboboxValue ?? "Select"}</span><IconChevronDown class="p-0" />
</button>

<div class={classNames("card shadow-xl", addListClass)} data-popup="combobox">
	<ListBox rounded="rounded-none">
		{#each items as item}
			<ListBoxItem bind:group={comboboxValue} name={name} value={item}>
				<span class={textClass}>{item}</span>
			</ListBoxItem>
		{/each}
	</ListBox>
	<!-- Arrow -->
	<div class="arrow bg-surface-100-800-token" />
</div>