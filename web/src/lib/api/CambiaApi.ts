import type { CambiaResponse } from "$lib/types/CambiaResponse";
import { Unpackr } from 'msgpackr';
import * as bigintConversion from 'bigint-conversion';
import { processedStore, pendingStore, fileMap } from "$lib/LogStore";
import { dev } from "$app/environment";
import { XXH64 } from 'xxh3-ts';
import { hexify } from "$lib/utils";

const PORT = dev ? 3031 : 3030;

export async function getRipInfoJson(files: FileList | undefined): Promise<CambiaResponse> {
    const reader: FileReader = new FileReader();

    return new Promise((resolve, reject) => {
        let byteArray: Uint8Array = new Uint8Array();

        if (!files || files.length <= 0) {
            reject(new Error("FileList is empty"));
        }

        // TODO: Upload progress
        // TODO: File rejection criteria
        const file = files?.[0];
        reader.onload = () => {
            const arrayBuffer: ArrayBuffer = reader.result as ArrayBuffer;
            byteArray = new Uint8Array(arrayBuffer);
        }

        reader.readAsArrayBuffer(file as File);

        // TODO: Not a fan of the nested mess here, need to revisit
        reader.onloadend = async () => {
            const options: RequestInit = {
                method: "POST",
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/octet-stream',
                },
                body: byteArray,
            }
            const request = new Request(`http://127.0.0.1:${PORT}/api/v1/upload`);

            try {
                const res = await fetch(request, options);
                const infoJson = await res.json();

                if (res.ok) {
                    if (infoJson.hasOwnProperty.call('error')) {
                        reject(new Error(`Error ${infoJson['error']} from server`));
                    }
                    resolve(infoJson);
                } else {
                    reject(new Error(`Error ${res.status} from server`));
                }
            } catch (error) {
                reject(error);
            }
        }
    });
}

export async function getRipInfoJsonMulti(files: FileList | undefined) {
    const ws: WebSocket = new WebSocket(`ws://localhost:${PORT}/ws/v1/upload_multi`);
    const unpackr = new Unpackr( {useRecords: false} );

    ws.addEventListener('message', (ev) => {
        if (ev.data instanceof Blob) {
            const r = new FileReader();
            r.onload = () => {
                const buf: ArrayBuffer = r.result as ArrayBuffer;
                const res = unpackr.unpack(new Uint8Array(buf)) as CambiaResponse;
                if (res.id) {
                    processedStore.update(p => {
                        p.push(res);
                        return p;
                    })
                }
            };
            r.readAsArrayBuffer(ev.data);
        }
    });

    ws.onopen = () => {
        const fileArray = Array.from(files || []);
        let bArr: Uint8Array = new Uint8Array();
        
        fileArray.forEach(f => {
            const r: FileReader = new FileReader();
            r.onload = () => {
                const aBuf: ArrayBuffer = r.result as ArrayBuffer;
                bArr = new Uint8Array(aBuf);
            }

            r.readAsArrayBuffer(f as File);
            
            r.onloadend = async () => {
                const hash: Uint8Array = (new Uint8Array(bigintConversion.bigintToBuf(XXH64(Buffer.from(bArr)), true))).subarray(0, 8); // clamp to 64-bit
                const hashHex = hexify(Array.from(hash)).toLowerCase();
                const tmp: Uint8Array = new Uint8Array(hash.length + bArr.length);
                
                console.log(hashHex);
                fileMap.set(hashHex, f.name);
                pendingStore.update(l => {
                    l.push(hashHex);
                    return l;
                });
                
                tmp.set(hash);
                tmp.set(bArr, hash.length);
                ws.send(tmp);
            }
        })
    }

    ws.onclose = () => {
        console.log("WS closed");
    }
}