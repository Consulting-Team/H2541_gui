<script>
import Menu from "./menu.svelte";
import { listen } from "@tauri-apps/api/event";
import Modal from "./modal.svelte";

let { children } = $props();
let showModal = $state(false);

listen('onProgress', event => {
    if (event.payload) {
        console.log('on processing...');
        showModal = true;
    } else {
        console.log('completed');
        showModal = false;
    }
});
</script>

<main>
    <nav class="nav-bar">
        <Menu></Menu>
    </nav>
    {@render children()}
</main>

<Modal bind:showModal={showModal}></Modal>

<style>
:global(html, body) {
    height: 100%;
    margin: 0;
    padding: 0;
}

main {
    display: flex;
    height: 100%;
    margin: 0;
}

.nav-bar {
    box-sizing: border-box;
    width: 30%;
    border-right: 1px solid black;
    min-width: 30vw;
}

/* .left-section {
    height: 100%;
} */

:global {
    .container {
        margin: 0;
        padding-top: 5vh;
        display: block;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    a {
        font-weight: 500;
        color: #646cff;
        text-decoration: inherit;
    }

    a:hover {
        color: #535bf2;
    }

    h1 {
        text-align: center;
    }

    input,
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        /* font-weight: 500; */
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
        cursor: pointer;
        padding: 0;
    }

    button:hover {
        border-color: #396cd8;
    }
    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    textarea {
        border-radius: 4px;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition: border-color 0.25s;
    }

    input,
    button, textarea  {
        outline: none;
    }
}
</style>