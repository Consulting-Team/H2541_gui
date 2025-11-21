<script>
    let {msg = $bindable()} = $props();
    /** @type {HTMLTextAreaElement} */
    let textArea;

    export function toBottom() {
        textArea.scrollTop = textArea.scrollHeight;
    }

    $effect(() => {
        console.log(typeof(msg));
        toBottom();
    });

</script>

<textarea bind:this={textArea} bind:value={msg} readonly></textarea>

<style>
textarea {
    border: 1px solid black;
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    font-family: 'Courier New', Courier, monospace;
    overflow-y: auto;
    white-space: nowrap;
    text-indent: 1em;
}
</style>