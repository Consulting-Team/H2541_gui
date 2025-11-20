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
    <h1 class="one">Font1</h1>
    <h1 id="two">Font2</h1>
    <h1 id="three">Font3</h1>
    <button onclick={askHandler}>ask</button>
    <button onclick={msgHandler}>msg</button>
</div>

<Modal showModal={showModal}></Modal>

<style>
@font-face {
    font-family: 'Source Code Pro Italic';
    src: url('/fonts/Source_Code_Pro/SourceCodePro-Italic-VariableFont_wght.ttf');
}

@font-face {
    font-family: 'Source Code Pro';
    src: url('/fonts/Source_Code_Pro/SourceCodePro-VariableFont_wght.ttf');
}

button {
    margin: 0 1em;
    width: 4em;
    height: 2.5em;
}

#two {
    font-family: 'Source Code Pro';
    background-color: aliceblue;
}

#three {
    font-family: 'Source Code Pro Italic';
    background-color: aliceblue;
}
</style>