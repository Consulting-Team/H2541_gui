<script>
let { action } = $props();

let activate = $state(true);

export function activateBtn() {
    activate = true;
}

export function deactivateBtn() {
    console.log('deactivate btn');
    activate = false;
}

</script>

<button disabled={!activate} onclick={ action }>Export</button>

<style>
button {
    width: 8em;
    height: 3.1em;
}

button:disabled {
    opacity: 0.5;                    /* 흐릿하게 */
    cursor: not-allowed;             /* 금지 커서 */
    pointer-events: none;            /* 클릭 무시 (안전하게) */
    background-color: #cccccc;       /* 회색 배경 */
    color: #666666;                  /* 어두운 텍스트 */
    border: 1px solid #999999;
    box-shadow: none;
}

/* 선택적으로: hover/focus 시에도 비활성 상태 유지 */
button:disabled:hover,
button:disabled:focus {
  background-color: #cccccc;
  color: #666666;
  transform: none;
}

</style>