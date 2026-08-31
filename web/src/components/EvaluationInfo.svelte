<script lang="ts">
    import { RecursiveTreeView, type TreeViewNode } from "@skeletonlabs/skeleton";
    import slugify from 'slugify';
	import Card from "./frags/Card.svelte";
    import type { EvaluationCombined } from "$lib/types/EvaluationCombined";
    import EvaluationUnitSegment from "./frags/treeview/EvaluationUnitSegment.svelte";
    import EvaluatorSegment from "./frags/treeview/EvaluatorSegment.svelte";
	import EvaluatorLead from "./frags/treeview/EvaluatorLead.svelte";
	import EvaluationUnitScopeSegment from "./frags/treeview/EvaluationUnitScopeSegment.svelte";
	import { asLegacyComponent, evaluationUnitScopeStringify } from "$lib/utils";
	import EvaluationUnitNoneSegment from "./frags/treeview/EvaluationUnitNoneSegment.svelte";
	import type { EvaluationUnitAggregate } from "$lib/types/EvaluationUnitAggregate";
	import type { ParsedLogCombined } from "$lib/types/ParsedLogCombined";

    interface Props {
        logs: ParsedLogCombined;
        combinedEvals: EvaluationCombined[];
        selectedLogIdx: number;
    }

    let { logs, combinedEvals, selectedLogIdx }: Props = $props();

    const tree: { nodes: TreeViewNode[], expanded: string[] } = $derived.by(() => {
        const evaluationCombined = combinedEvals.filter(e => e.evaluator === "OPS")[0];
        const evaluation_units = evaluationCombined.evaluations[selectedLogIdx].evaluation_units;
        const unitsByScope: { [key: string]: EvaluationUnitAggregate } = {};

        const treeViewNodes: TreeViewNode[] = [];
        const expandedNodes: string[] = [];

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
            lead: asLegacyComponent(EvaluatorLead),
            leadProps: {
                evaluator: evaluationCombined.evaluator
            },
            content: asLegacyComponent(EvaluatorSegment),
            contentProps: {
                evaluator: evaluationCombined.evaluator,
                score: evaluationCombined.evaluations[selectedLogIdx].score,
                combinedScore: evaluationCombined.combined_score
            },
            children: evaluation_units.length == 0 ? [{
                id: "evaluation-unit-none",
                content: asLegacyComponent(EvaluationUnitNoneSegment),
                contentProps: {
                    checksum: logs.parsed_logs[selectedLogIdx].checksum
                }
            }] : Object.keys(unitsByScope).map(scope => (
            {
                id: unitsByScope[scope].slug,
                content: asLegacyComponent(EvaluationUnitScopeSegment),
                contentProps: {
                    scope: scope
                },
                children: unitsByScope[scope].evaluation_units.map(unit => ({
                    id: slugify(unit.data.field),
                    content: asLegacyComponent(EvaluationUnitSegment),
                    contentProps: {
                        evaluation_unit: unit
                    },
                }))
            }))
        });

        return { nodes: treeViewNodes, expanded: expandedNodes };
    });

    const treeViewNodes = $derived(tree.nodes);
    const expandedNodes = $derived(tree.expanded);
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