import type { CambiaResponse } from "$lib/types/CambiaResponse";
import { Unpackr } from 'msgpackr';
import * as bigintConversion from 'bigint-conversion';
import { errorStore, hashIndexLookup, processedCount, responseStore, updateStat, updateUnknown } from "$lib/LogStore";
import { dev } from "$app/environment";
import { XXH64 } from 'xxh3-ts';
import { clientError, hexify, isCambiaError, isCambiaResponse, removeRoute } from "$lib/utils";
import type { CambiaError } from "$lib/types/CambiaError";
import { goto } from "$app/navigation";
import { page } from '$app/stores';
import { get } from "svelte/store";

export async function getRipInfoMpMulti(from: string | null, files: FileList | undefined) {
    const endpoint = `${location.protocol.startsWith("https") ? "wss" : "ws"}://${location.host}${removeRoute(location.pathname, from)}`;
    const ws: WebSocket = new WebSocket(`${dev ? "ws://localhost:3031" : endpoint}/ws/v1/upload_multi`);
    const unpackr = new Unpackr( {useRecords: false} );

    ws.onmessage = (ev) => {
        if (ev.data instanceof Blob) {
            const r = new FileReader();
            r.onload = () => {
                const buf: ArrayBuffer = r.result as ArrayBuffer;
                try {
                    const res = unpackr.unpack(new Uint8Array(buf)) as CambiaResponse | CambiaError;
                    if (isCambiaResponse(res)) {
                        updateStat(res);
                        responseStore.update(store => {
                            const indices = hashIndexLookup.get(hexify(res.id));
                            if (indices === undefined) {
                                console.error("Unknown log ID");
                                return store;
                            }
                            indices.forEach(idx => {
                                store[idx].status = "processed";
                                store[idx].content = res;
                            });
                            return store;
                        });
                    } else if (isCambiaError(res)) {
                        updateUnknown();
                        responseStore.update(store => {
                            const indices = hashIndexLookup.get(hexify(res.id));
                            if (indices !== undefined) {
                                indices.forEach(idx => {
                                    store[idx].status = "errored";
                                    store[idx].content = res;
                                });
                            }
                            return store;
                        });
                    }
                } catch (error) {
                    updateUnknown();
                } finally {
                    processedCount.update(p => p + 1);
                }
            };
            r.readAsArrayBuffer(ev.data);
        }
    };

    ws.onopen = () => {
        const fileArray = Array.from(files || []);
        let bArr: Uint8Array = new Uint8Array();
        
        fileArray.forEach((f, idx) => {
            const r: FileReader = new FileReader();
            r.onload = () => {
                const aBuf: ArrayBuffer = r.result as ArrayBuffer;
                bArr = new Uint8Array(aBuf);
            }

            r.readAsArrayBuffer(f as File);
            
            r.onloadend = async () => {
                const hashPadded = new Uint8Array(8);
                const hash: Uint8Array = (new Uint8Array(bigintConversion.bigintToBuf(XXH64(Buffer.from(bArr)), true))).subarray(0, 8); // clamp to 64-bit
                hashPadded.set(hash, hashPadded.length - hash.length);

                const hashHex = hexify(Array.from(hashPadded));
                const tmp: Uint8Array = new Uint8Array(hashPadded.length + bArr.length);
                
                if (hashIndexLookup.has(hashHex)) {
                    hashIndexLookup.get(hashHex)?.push(idx);
                } else {
                    hashIndexLookup.set(hashHex, [idx]);
                }
                
                if (bArr.length > 3145728) {
                    ws.dispatchEvent(clientError("Files over 3 MiB not allowed.", Array.from(hashPadded)));
                    return;
                }

                tmp.set(hashPadded);
                tmp.set(bArr, hashPadded.length);
                ws.send(tmp);
            }
        })
    }

    ws.onerror = () => {
        const error: CambiaError = {
            id: [],
            message: "Connection to the API failed."
        };
        errorStore.set(error);
        goto(`${removeRoute(location.pathname, get(page).route.id)}/error`);
    }
}