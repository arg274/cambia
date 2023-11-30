import type { ToastSettings, ToastStore } from '@skeletonlabs/skeleton';
import anime from 'animejs';
import Color from 'colorjs.io';

// const toastStore = getToastStore();

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
    return data.map(x => x.toString(16).padStart(2, '0')).join('');
}

export function getScoreVariant(score: string): string {
    if (isNumeric(score)) {
        const numScore = parseFloat(score);
        switch (true) {
            case numScore === 100:
                return "variant-soft-success";
            case numScore >= 50:
                return "variant-soft-warning";
            case numScore < 50:
                return "variant-soft-error";
        }
    }
    return "variant-soft-primary";
}