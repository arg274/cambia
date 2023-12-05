import { writable } from "svelte/store";
import type { CambiaResponse } from "$lib/types/CambiaResponse";

// FIXME: No way to know pending/failed requests
export const processedStore = writable(new Array<CambiaResponse>());
export const pendingStore = writable(new Array<string>());
export const fileMap: Map<string, string> = new Map();
export const fileListStore = writable<FileList | undefined>();