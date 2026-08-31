import { createToaster } from '@skeletonlabs/skeleton-svelte';

/**
 * App-wide toaster. Skeleton v5 replaces v2's context-based `getToastStore()`
 * with a plain store created once and rendered by a `<Toast.Group>`; the layout
 * mounts that group, and anything can push to this store.
 */
export const toaster = createToaster({
	placement: 'top-end',
	duration: 3000
});
