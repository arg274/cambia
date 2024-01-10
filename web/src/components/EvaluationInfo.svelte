<script lang="ts">
    import { RecursiveTreeView, type TreeViewNode } from "@skeletonlabs/skeleton";
    import slugify from 'slugify';
	import Card from "./frags/Card.svelte";
    import type { EvaluationCombined } from "$lib/types/EvaluationCombined";
    import DeductionSegment from "./frags/treeview/DeductionSegment.svelte";
    import EvaluatorSegment from "./frags/treeview/EvaluatorSegment.svelte";
	import EvaluatorLead from "./frags/treeview/EvaluatorLead.svelte";
	import DeductionCategorySegment from "./frags/treeview/DeductionCategorySegment.svelte";
	import { deductionCategoryStringify } from "$lib/utils";
	import DeductionNoneSegment from "./frags/treeview/DeductionNoneSegment.svelte";
	import type { DeductionAggregate } from "$lib/types/DeductionAggregate";

    export let combinedEvals: EvaluationCombined[];
    export let selectedLogIdx: number;

    let evaluationCombined = combinedEvals.filter(e => e.evaluator === "OPS")[0];
    let deductions = evaluationCombined.evaluations[selectedLogIdx].deductions;
    let deductionsByCategory: { [key: string]: DeductionAggregate } = {};

    let treeViewNodes: TreeViewNode[] = [];
    let expandedNodes: string[] = [];

    deductions.forEach(deduction => {
        let categoryKey = deductionCategoryStringify(deduction.data.category);
        if (!deductionsByCategory[categoryKey]) {
            const slug = slugify(categoryKey);
            expandedNodes.push(slug);
            deductionsByCategory[categoryKey] = { slug: slug, deductions: [ deduction ] };
        } else {
            deductionsByCategory[categoryKey].deductions.push(deduction);
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
        children: deductions.length == 0 ? [{
            id: "deduction-none",
            content: DeductionNoneSegment
        }] : Object.keys(deductionsByCategory).map(category => (
        {
            id: deductionsByCategory[category].slug,
            content: DeductionCategorySegment,
            contentProps: {
                category: category
            },
            children: deductionsByCategory[category].deductions.map(deduction => ({
                id: slugify(deduction.data.field),
                content: DeductionSegment,
                contentProps: {
                    deduction: deduction
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