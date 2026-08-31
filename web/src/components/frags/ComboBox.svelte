<script lang="ts">
	import { ListBox, ListBoxItem, popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import classNames from 'classnames';
	import { onMount } from "svelte";
	
	import IconChevronDown from '~icons/carbon/chevron-down';


	interface Props {
		addListClass?: string;
		addTriggerClass?: string;
		textClass?: string;
		items?: string[];
		name?: string;
		value?: string;
	}

	let {
		addListClass = "",
		addTriggerClass = "",
		textClass = "",
		items = [],
		name = "",
		value = $bindable("")
	}: Props = $props();

    let popupCombobox: PopupSettings = {
        event: 'focus-click',
        target: 'combobox',
        placement: 'bottom',
        // Close the popup when the item is clicked
        closeQuery: '.listbox-item'
    };

	onMount(async () => {
		if (items.length !== 0) {
			value = items[0];
		}
	})
</script>

<button class={classNames("btn bg-surface-200-700-token pr-2", addTriggerClass)} use:popup={popupCombobox}>
	<span class={textClass}>{value ?? "Select"}</span><IconChevronDown class="p-0" />
</button>

<div class={classNames("card shadow-xl z-10", addListClass)} data-popup="combobox">
	<ListBox rounded="rounded-none">
		{#each items as item}
			<ListBoxItem bind:group={value} name={name} value={item}>
				<span class={textClass}>{item}</span>
			</ListBoxItem>
		{/each}
	</ListBox>
	<div class="arrow bg-surface-100-800-token"></div>
</div>