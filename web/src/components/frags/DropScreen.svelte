<script lang="ts">
    import classNames from 'classnames';
    import IconCloudUpload from '~icons/carbon/cloud-upload';
    import type { Snippet } from 'svelte';
    import type { HTMLInputAttributes } from 'svelte/elements';

    interface Props extends Omit<HTMLInputAttributes, 'files' | 'class' | 'onchange' | 'children'> {
        files?: FileList;
        class?: string;
        onchange?: () => void;
        children?: Snippet;
    }

    let {
        files = $bindable(),
        class: className,
        onchange,
        children,
        ...rest
    }: Props = $props();

    let focused: boolean = $state(false);

    const baseClass: string = "fixed top-0 left-0 right-0 group flex flex-col justify-center items-center w-screen h-screen z-max";
    const focusClass = "visible ease-in duration-200 bg-surface-100-800";
    const blurClass = "invisible ease-out duration-200";
    const textBaseClass = "mt-2 mb-2";
    const textFocusClass = "";
    const textBlurClass = "invisible";
    const svgBaseClass = "mt-4 h-36 w-36";
    const svgFocusClass = "";
    const svgBlurClass = "hidden";

    let input: HTMLInputElement | undefined = $state();

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
        ev.preventDefault();
        if (!ev.dataTransfer) return;
        blur();

        let dtFiles = ev.dataTransfer.types.filter(t => t === 'Files')

        if (dtFiles.length <= 0) return;

        files = ev.dataTransfer?.files;
        onchange?.();
    }
</script>

<svelte:window ondragenter={(ev) => { ev.stopPropagation(); dragenter(ev); }} />
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<label
    class={classNames(baseClass, focused ? focusClass: blurClass, className)}
    tabIndex="-1"
    onclick={(ev) => ev.preventDefault()}
    ondragleave={(ev) => { ev.preventDefault(); blur(); }}
    ondragover={(ev) => ev.preventDefault()}
    ondrop={drop}>
    <div class="flex flex-col justify-center items-center pointer-events-none">
        <IconCloudUpload class={classNames(svgBaseClass, focused ? svgFocusClass : svgBlurClass)} />
        <p class={classNames(textBaseClass, focused ? textFocusClass : textBlurClass)}><span class="font-bold">Drag and drop</span> log files here</p>
    </div>
    <input {...rest} bind:files bind:this={input} type="file" class="hidden" onchange={() => onchange?.()} />
</label>
<!--
    Hiding this with `display: none` took the page content out of flow, so the
    footer slid up into the freed space for as long as the drag lasted. The
    overlay covers it either way; keeping the box means nothing reflows.
-->
<div class={classNames("h-full", focused ? "invisible pointer-events-none" : "visible")}>
    {@render children?.()}
</div>
