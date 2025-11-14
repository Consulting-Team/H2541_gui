<script>
let {showModal = $bindable()} = $props();

/** @type {HTMLDialogElement} */
let dialog;

export function closeModal() {
    showModal = false;
    dialog.close();
}

$effect(() => {
    if (showModal) {
        dialog.showModal();
    } else {
        dialog.close();
    }
});

</script>

<dialog bind:this={dialog} onclose={() => (showModal = false)}>
    <h1>Hello this is a modal page.</h1>
    <progress value="0.5"></progress>
    <button onclick={closeModal}>close</button>
</dialog>