import type { CambiaError } from "./CambiaError";
import type { CambiaResponse } from "./CambiaResponse";

export interface ResponseEntry { filename: string, status: string, content: CambiaResponse | CambiaError | null }