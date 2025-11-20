<script>
let {showModal = $bindable()} = $props();

/** @type {HTMLDialogElement} */
let dialog;

// export function closeModal() {
//     showModal = false;
//     dialog.close();
// }

/** @param {Event} event */
function cancelHandle(event) {
    event.preventDefault();
}

$effect(() => {
    if (showModal) {
        dialog.showModal();
    } else {
        dialog.close();
    }
});

</script>

<dialog bind:this={dialog} onclose={() => (showModal = false)} oncancel={cancelHandle}>
    <h1>On progress</h1>
    <progress></progress>
</dialog>

<style>
    dialog {
        width: 50vh;
    }
    progress {
        width: 100%;
    }
</style>