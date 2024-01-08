import { writable } from "svelte/store";
import type { ResponseEntry } from "./types/ResponseEntry";
import type { CambiaResponse } from "./types/CambiaResponse";
import type { CambiaError } from "./types/CambiaError";

// FIXME: No way to know pending/failed requests
export const processedCount = writable(0);
export const responseStore = writable(new Array<ResponseEntry>());
export const fileListStore = writable<FileList | undefined>();
export const hashIndexLookup = new Map<string, number[]>();
export const perfectCount = writable(0);
export const warningCount = writable(0);
export const badCount = writable(0);
export const unknownCount = writable(0);
export const errorStore = writable<CambiaError | null>(null);

export function initialiseResponseStore(files: FileList | undefined) {
    hashIndexLookup.clear();
    processedCount.set(0);
    perfectCount.set(0);
    warningCount.set(0);
    badCount.set(0);
    unknownCount.set(0);
    responseStore.set(Array.from(files || []).map(file => <ResponseEntry> {filename: file.name, status: "queued", content: null}));
}

export function updateUnknown() {
    unknownCount.update(c => c + 1);
}

export function updateStat(content: CambiaResponse) {
    // TODO: Score-based evaluation is dumb; switch to Cambia eval in future
    const score = parseInt(content.evaluation_combined.filter(x => x.evaluator === 'OPS')[0].combined_score);
    switch (true) {
        case (score < 0):
            badCount.update(c => c + 1);
            break;
        case (score < 100):
            warningCount.update(c => c + 1);
            break;
        case (score == 100):
            perfectCount.update(c => c + 1);
            break;
        default:
            updateUnknown();
            break;
    }
}