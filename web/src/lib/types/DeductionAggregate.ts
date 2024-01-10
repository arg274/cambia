import type { Deduction } from "./Deduction";

export interface DeductionAggregate {
    slug: string,
    deductions: Deduction[]
}