<script lang="ts">
    import { RecursiveTreeView, type TreeViewNode } from "@skeletonlabs/skeleton";
    import slugify from 'slugify';
	import Card from "./frags/Card.svelte";
    import type { EvaluationCombined } from "$lib/types/EvaluationCombined";
    import EvaluationUnitSegment from "./frags/treeview/EvaluationUnitSegment.svelte";
    import EvaluatorSegment from "./frags/treeview/EvaluatorSegment.svelte";
	import EvaluatorLead from "./frags/treeview/EvaluatorLead.svelte";
	import EvaluationUnitScopeSegment from "./frags/treeview/EvaluationUnitScopeSegment.svelte";
	import { evaluationUnitScopeStringify } from "$lib/utils";
	import EvaluationUnitNoneSegment from "./frags/treeview/EvaluationUnitNoneSegment.svelte";
	import type { EvaluationUnitAggregate } from "$lib/types/EvaluationUnitAggregate";
	import type { ParsedLogCombined } from "$lib/types/ParsedLogCombined";

    export let logs: ParsedLogCombined;
    export let combinedEvals: EvaluationCombined[];
    export let selectedLogIdx: number;

    let evaluationCombined = combinedEvals.filter(e => e.evaluator === "OPS")[0];
    let evaluation_units = evaluationCombined.evaluations[selectedLogIdx].evaluation_units;
    let unitsByScope: { [key: string]: EvaluationUnitAggregate } = {};

    let treeViewNodes: TreeViewNode[] = [];
    let expandedNodes: string[] = [];

    evaluation_units.forEach(unit => {
        let scopeKey = evaluationUnitScopeStringify(unit.data.scope);
        if (!unitsByScope[scopeKey]) {
            const slug = slugify(scopeKey);
            expandedNodes.push(slug);
            unitsByScope[scopeKey] = { slug: slug, evaluation_units: [ unit ] };
        } else {
            unitsByScope[scopeKey].evaluation_units.push(unit);
        }
    });

    treeViewNodes.push({
        id: evaluationCombined.evaluator,
        lead: EvaluatorLead,
        leadProps: {
            evaluator: evaluationCombined.evaluator
        },
        content: EvaluatorSegment,
        contentProps: {
            evaluator: evaluationCombined.evaluator,
            score: evaluationCombined.evaluations[selectedLogIdx].score,
            combinedScore: evaluationCombined.combined_score
        },
        children: evaluation_units.length == 0 ? [{
            id: "evaluation-unit-none",
            content: EvaluationUnitNoneSegment,
            contentProps: {
                checksum: logs.parsed_logs[selectedLogIdx].checksum
            }
        }] : Object.keys(unitsByScope).map(scope => (
        {
            id: unitsByScope[scope].slug,
            content: EvaluationUnitScopeSegment,
            contentProps: {
                scope: scope
            },
            children: unitsByScope[scope].evaluation_units.map(unit => ({
                id: slugify(unit.data.field),
                content: EvaluationUnitSegment,
                contentProps: {
                    evaluation_unit: unit
                },
            }))
        }))
    });
</script>

<!-- TODO: This will need a massive overhaul to handle colours and goto highlighting -->
<Card header="Evaluations">
    <RecursiveTreeView
        nodes={treeViewNodes}
        expandedNodes={expandedNodes}
        indent="ml-1 my-2"
        padding="pl-2 py-0"
        hyphenOpacity="opacity-0">
    </RecursiveTreeView>
</Card>