import type { ToastSettings, ToastStore } from '@skeletonlabs/skeleton';
import anime from 'animejs';
import Color from 'colorjs.io';
import type { CambiaResponse } from './types/CambiaResponse';
import type { CambiaError } from './types/CambiaError';
import { Packr } from 'msgpackr';
import type { DeductionCategory } from './types/DeductionCategory';

const packr = new Packr({ useRecords: false });

export function toHeaderId(header: string): string {
    return header.trim().replaceAll(" ", "-").toLowerCase();
}

export function toInfoHeaderId(header: string): string {
    return header.trim().replaceAll(" ", "-").toLowerCase() + "-info-header";
}

export function toInfoId(header: string): string {
    return header.trim().replaceAll(" ", "-").toLowerCase() + "-info";
}

export function toCardId(header: string): string {
    return header.trim().replaceAll(" ", "-").toLowerCase() + "-card";
}

export function copySuccess(toastStore: ToastStore) {
    const t: ToastSettings = {
        message: 'Copied to clipboard',
        background: 'variant-glass-primary',
        timeout: 3000
    };
    toastStore.trigger(t);
}

export function showError(toastStore: ToastStore, err: CambiaError) {
    const t: ToastSettings = {
        message: err.message,
        background: 'variant-glass-error',
        timeout: 3000
    };
    toastStore.trigger(t);
}

export function getCssColor(cssVar: string): Color {
    return new Color(`rgb(${getComputedStyle(document.body).getPropertyValue("--" + cssVar)})`)
}

export function getCssColorHex(cssVar: string): string {
    return getCssColor(cssVar).toString({format: "hex"});
}

export function nonNullAssert<T>(a: T | undefined | null): T {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return a!;
}

export function castToType<T>(a: object): T {
    return a as T;
}

export function castToTypeArray<T>(a: object): T[] {
    return a as T[];
}

export function isNumeric(value: string): boolean {
    return /^-?\d+$/.test(value);
}

export function runAnimation(logoId: string) {
    anime({
        targets: `#${logoId} path`,
        strokeDashoffset: {
            value: [anime.setDashoffset, 0],
            easing: 'easeInOutSine',
            duration: 3000,
        },
        stroke: {
            value: getCssColorHex('color-primary-400'),
            easing: 'easeInOutSine',
            delay: 3000,
            duration: 5000,
        }
    });
}

export function hexify(data: number[]): string {
    return data.map(x => x.toString(16).padStart(2, '0')).join('').toLowerCase();
}

export function getScoreVariant(score: string): string {
    if (isNumeric(score)) {
        const numScore = parseFloat(score);
        switch (true) {
            case numScore == 100:
                return "variant-soft-success";
            case numScore >= 50:
                return "variant-soft-warning";
            case numScore < 50:
                return "variant-soft-error";
        }
    }
    return "variant-soft-surface";
}

export function getInfoOverviewPopoverText(miniName: string) {
    const mapping: Record<string, string> = {
        "ACS": "Accurate Stream",
        "DAC": "Defeat Audio Cache",
        "C2E": "C2 Enabled",
        "FMS": "Fill Missing Samples With Silence",
        "DSB": "Delete Silent Blocks",
        "NSC": "Null Samples in CRC",
        "T&C": "Test and Copy",
        "NML": "Normalisation"
    };
    return mapping[miniName];
}

export function clientError (message: string, id: Array<number> = []): MessageEvent {
    const err: CambiaError = {
        id,
        message
    }
    const packed = packr.pack(err);
    return new MessageEvent("message", { data: new Blob([packed]) });
}

export function isCambiaResponse(res: CambiaResponse | CambiaError): res is CambiaResponse {
    return "parsed" in res && res.id.length > 0;
}

export function isCambiaError(res: CambiaResponse | CambiaError): res is CambiaError {
    return "message" in res;
}

export function trimLeftChar(str: string, ch: string) {
    let start = 0;
    const end = str.length;

    while (start < end && str[start] === ch) {
        ++start;
    }

    return start > 0 ? str.substring(start, end) : str;
}

export function deductionCategoryStringify(category: DeductionCategory) {
    return typeof category === 'string' ? category : `Track ${category.Track}`;
}