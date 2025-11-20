<script>
import Modal from "$lib/modal.svelte";
import ReportIOPanel from "$lib/reportIOPanel.svelte";

let showModal = $state(false);
</script>

<article class="container">
    <ReportIOPanel bind:showModal={showModal} repoType={"voyage"}></ReportIOPanel>
</article>

<Modal bind:showModal={showModal}></Modal>

<style>
article {
    box-sizing: border-box;
    width: 100%;
    padding: 1em 3vw;
}
</style>