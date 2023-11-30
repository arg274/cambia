<script>
    import IconMachineLearningModel from '~icons/carbon/machine-learning-model';
	import Card from "./frags/Card.svelte";
	import ComboBox from "./frags/ComboBox.svelte";

</script>
<Card header="Album Info" addClass="col-span-2">
    <div class="flex mt-2 justify-between">
        <div class="flex items-center">
            <img width="100" height="100" alt="Album by Artist" src="https://is3-ssl.mzstatic.com/image/thumb/Music7/v4/ea/2c/36/ea2c360f-1674-865a-e93a-bcc499bdf381/4511820-93417.jpg/100x100bb.webp" />
            <div class="ml-4 flex flex-col">
                <span class="text-md">Kikuo</span>
                <span class="text-2xl font-bold">Kikuo Miku 4</span>
            </div>
        </div>
        <a type="button" class="btn-icon bg-initial hover:variant-soft self-start" href="https://musicbrainz.org/release/8242a7d2-4269-4ec6-919d-9328dbeac13b" target="_blank"><IconMachineLearningModel /></a>
    </div>
    <div class="mt-4 flex justify-between items-end">
        <span><ComboBox addTriggerClass="h-6" textClass="text-sm font-mono" items={["8242a7d2-4269-4ec6-919d-9328dbeac13b", "fdf81975-402a-4bbe-9c13-2aa0d0e62e40"]} /></span> <!-- TODO: Combobox for releaseMBID goes here -->
        <span class="text-xs">KSW-0009</span>
    </div>
</Card>