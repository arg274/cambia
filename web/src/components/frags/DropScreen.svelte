<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import classNames from 'classnames';
    import IconCloudUpload from '~icons/carbon/cloud-upload';

    export let value: string = '';
    export let files: FileList | undefined;

    const dispatch = createEventDispatcher();
    let focused: boolean = false;

    const baseClass: string = `
        fixed
        top-0
        left-0
        right-0
        group
        flex
        flex-col
        justify-center
        items-center
        w-screen
        h-screen
        z-max
    `;
    
    const focusClass = `
        visible ease-in duration-200
        bg-surface-100-800-token
    `;

    const blurClass = `
        invisible ease-out duration-200
    `;

    const textBaseClass = `
        mt-2
        mb-2
    `;

    const textFocusClass = "";

    const textBlurClass = `
        invisible
    `;

    const svgBaseClass = `
        mt-4
        h-36
        w-36
    `;

    const svgFocusClass = "";

    const svgBlurClass = `
        hidden
    `;
    
    let input: HTMLInputElement;

    function focus() {
        focused = true;
    }

    function blur() {
        focused = false;
    }

    function dragenter(ev: DragEvent) {
        ev.preventDefault();
        if (ev.dataTransfer && ev.dataTransfer.types.filter(t => t === 'Files').length > 0) {
            focus();
        }
    }

    function drop(ev: DragEvent) {
        blur();
        if (ev.dataTransfer && ev.dataTransfer.types.filter(t => t === 'Files').length > 0) {
            files = ev.dataTransfer?.files;
            dispatch('change');
        }
    }
</script>

<svelte:window on:dragenter|stopPropagation={dragenter} />
<label
    class={classNames(baseClass, focused ? focusClass: blurClass, $$props.class)}
    tabIndex="-1"
    on:click|preventDefault
    on:keydown
    on:focus
    on:blur
    on:mouseenter
    on:mouseleave
    on:mouseover
    on:dragenter
    on:dragleave|preventDefault={blur}
    on:dragover|preventDefault
    on:drop|preventDefault={drop}>
    <div class="flex flex-col justify-center items-center pointer-events-none">
        <IconCloudUpload class={classNames(svgBaseClass, focused ? svgFocusClass : svgBlurClass)} />
        <p class={classNames(textBaseClass, focused ? textFocusClass : textBlurClass)}><span class="font-bold">Drag and drop</span> log files here</p>
    </div>
    <input {...$$restProps} bind:value bind:files bind:this={input} type="file" class="hidden" on:change on:click />
</label>
<div class={focused ? classNames("hidden", "pointer-events-none") : "visible h-full"}>
    <slot />
</div>