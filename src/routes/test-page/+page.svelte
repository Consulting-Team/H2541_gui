<script>
import Modal from "../modal.svelte";
import { ask, message } from "@tauri-apps/plugin-dialog";

let showModal = $state(false);


async function askHandler() {
    const answer = await ask('ask test', {
        title: 'Tauri',
        kind: 'warning'
    });
}

async function msgHandler() {
    await message('Message', {
        title: 'Tauri',
        kind: "error"
    });
}


</script>

<div class="container">
    <h1>Test</h1>
    <button onclick={askHandler}>ask</button>
    <button onclick={msgHandler}>msg</button>
</div>

<Modal showModal={showModal}></Modal>

<style>
    button {
        margin: 0 1em;
        width: 4em;
        height: 2.5em;
    }
</style>