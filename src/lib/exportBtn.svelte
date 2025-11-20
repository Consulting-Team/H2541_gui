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
    background-color: #FAFBFC;
}

button:disabled {
    opacity: 0.5;                    /* 흐릿하게 */
    cursor: not-allowed;             /* 금지 커서 */
    pointer-events: none;            /* 클릭 무시 (안전하게) */
    background-color: #cccccc;     /* 회색 배경
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


/* CSS */
.button-4 {
  appearance: none;
  background-color: #FAFBFC;
  border: 1px solid rgba(27, 31, 35, 0.15);
  border-radius: 6px;
  box-shadow: rgba(27, 31, 35, 0.04) 0 1px 0, rgba(255, 255, 255, 0.25) 0 1px 0 inset;
  box-sizing: border-box;
  color: #24292E;
  cursor: pointer;
  display: inline-block;
  /* font-family: -apple-system, system-ui, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji"; */
  font-size: 14px;
  font-weight: 500;
  line-height: 20px;
  list-style: none;
  padding: 6px 16px;
  position: relative;
  transition: background-color 0.2s cubic-bezier(0.3, 0, 0.5, 1);
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
  vertical-align: middle;
  white-space: nowrap;
  word-wrap: break-word;
}

.button-4:hover {
  background-color: #F3F4F6;
  text-decoration: none;
  transition-duration: 0.1s;
}

.button-4:disabled {
  background-color: #FAFBFC;
  border-color: rgba(27, 31, 35, 0.15);
  color: #959DA5;
  cursor: default;
}

.button-4:active {
  background-color: #EDEFF2;
  box-shadow: rgba(225, 228, 232, 0.2) 0 1px 0 inset;
  transition: none 0s;
}

.button-4:focus {
  outline: 1px transparent;
}

.button-4:before {
  display: none;
}

.button-4:-webkit-details-marker {
  display: none;
}
</style>