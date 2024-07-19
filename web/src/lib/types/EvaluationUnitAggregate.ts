import type { EvaluationUnit } from './EvaluationUnit';

export interface EvaluationUnitAggregate {
	slug: string;
	evaluation_units: EvaluationUnit[];
}
