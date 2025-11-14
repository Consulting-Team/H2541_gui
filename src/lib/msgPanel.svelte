<script>
    let {msg = $bindable()} = $props();
    /** @type {HTMLTextAreaElement} */
    let textArea;

    export function toBottom() {
        textArea.scrollTop = textArea.scrollHeight;
    }

    $effect(() => {
        // console.log(msg);
        console.log(typeof(msg));
        toBottom();
    });

</script>

<textarea bind:this={textArea} bind:value={msg} readonly></textarea>

<style>
textarea {
    border: 1px solid black;
    width: 100%;
    box-sizing: border-box;
    height: 20vh;
    font-family: 'Courier New', Courier, monospace;
    /* overflow-x: scroll; */
    overflow-y: scroll;
    white-space: nowrap;
    padding: 0;
}
</style>