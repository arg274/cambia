<script lang="ts">
    import { Collapsible } from "bits-ui";
    import IconChevronDown from '~icons/carbon/chevron-down';
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
	import type { EvaluationUnit } from "$lib/types/EvaluationUnit";
	import type { EvaluatorType } from "$lib/types/EvaluatorType";
	import type { Checksum } from "$lib/types/Checksum";

    interface Props {
        logs: ParsedLogCombined;
        combinedEvals: EvaluationCombined[];
        selectedLogIdx: number;
    }

    let { logs, combinedEvals, selectedLogIdx }: Props = $props();

    /**
     * Skeleton v5's TreeView is driven by a collection of plain data rather than
     * v2's `nodes` array of components, so each node carries the payload its
     * segment component needs and the markup below picks the renderer.
     */
    type EvalNode = {
        id: string;
        kind: 'root' | 'evaluator' | 'scope' | 'unit' | 'none';
        evaluator?: EvaluatorType;
        score?: string;
        combinedScore?: string;
        scope?: string;
        unit?: EvaluationUnit;
        checksum?: Checksum;
        children?: EvalNode[];
    };

    const tree: { root: EvalNode, expanded: string[] } = $derived.by(() => {
        const evaluationCombined = combinedEvals.filter(e => e.evaluator === "OPS")[0];
        const evaluation_units = evaluationCombined.evaluations[selectedLogIdx].evaluation_units;
        const unitsByScope: { [key: string]: EvaluationUnitAggregate } = {};
        const expanded: string[] = [];

        evaluation_units.forEach(unit => {
            const scopeKey = evaluationUnitScopeStringify(unit.data.scope);
            if (!unitsByScope[scopeKey]) {
                const slug = slugify(scopeKey);
                expanded.push(slug);
                unitsByScope[scopeKey] = { slug: slug, evaluation_units: [ unit ] };
            } else {
                unitsByScope[scopeKey].evaluation_units.push(unit);
            }
        });

        const children: EvalNode[] = evaluation_units.length == 0 ? [{
            id: "evaluation-unit-none",
            kind: 'none',
            checksum: logs.parsed_logs[selectedLogIdx].checksum
        }] : Object.keys(unitsByScope).map(scope => ({
            id: unitsByScope[scope].slug,
            kind: 'scope',
            scope: scope,
            children: unitsByScope[scope].evaluation_units.map(unit => ({
                id: `${unitsByScope[scope].slug}-${slugify(unit.data.field)}`,
                kind: 'unit',
                unit: unit
            } as EvalNode))
        } as EvalNode));

        return {
            root: {
                id: 'ROOT',
                kind: 'root',
                children: [{
                    id: evaluationCombined.evaluator,
                    kind: 'evaluator',
                    evaluator: evaluationCombined.evaluator,
                    score: evaluationCombined.evaluations[selectedLogIdx].score,
                    combinedScore: evaluationCombined.combined_score,
                    children: children
                }]
            } as EvalNode,
            expanded
        };
    });

</script>

{#snippet nodeBody(node: EvalNode)}
    {#if node.kind === 'evaluator'}
        <EvaluatorLead evaluator={node.evaluator!} />
        <div class="w-full">
            <EvaluatorSegment
                evaluator={node.evaluator!}
                score={node.score!}
                combinedScore={node.combinedScore!}
            />
        </div>
    {:else if node.kind === 'scope'}
        <div class="w-full"><EvaluationUnitScopeSegment scope={node.scope!} /></div>
    {:else if node.kind === 'unit'}
        <div class="w-full"><EvaluationUnitSegment evaluation_unit={node.unit!} /></div>
    {:else if node.kind === 'none'}
        <div class="w-full"><EvaluationUnitNoneSegment checksum={node.checksum!} /></div>
    {/if}
{/snippet}

{#snippet treeNode(node: EvalNode)}
    {#if node.children && node.children.length > 0}
        <Collapsible.Root open={tree.expanded.includes(node.id)} class="my-1">
            <Collapsible.Trigger
                class="group w-full flex items-center gap-4 pl-2 py-0 cursor-pointer text-left"
            >
                <IconChevronDown
                    class="shrink-0 w-3 fill-current transition-transform duration-200 group-data-[state=open]:rotate-180"
                />
                {@render nodeBody(node)}
            </Collapsible.Trigger>
            <Collapsible.Content class="ml-1 my-2">
                {#each node.children as child (child.id)}
                    {@render treeNode(child)}
                {/each}
            </Collapsible.Content>
        </Collapsible.Root>
    {:else}
        <!-- Leaves are informational, so they are not focus targets. -->
        <div class="flex items-center gap-4 pl-2 py-0 my-1">
            {@render nodeBody(node)}
        </div>
    {/if}
{/snippet}

<!-- TODO: This will need a massive overhaul to handle colours and goto highlighting -->
<Card header="Evaluations">
    <div class="w-full">
        {#each tree.root.children ?? [] as node (node.id)}
            {@render treeNode(node)}
        {/each}
    </div>
</Card>
