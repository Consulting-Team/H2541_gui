<script>
import { invoke } from "@tauri-apps/api/core";
import { save } from '@tauri-apps/plugin-dialog';
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import YAML from 'yaml';
import ExportBtn from '$lib/exportBtn.svelte';
import { gmt_list } from "$lib/data";
import MsgPanel from "$lib/msgPanel.svelte";
import Modal from "../modal.svelte";

const appWebView = getCurrentWebviewWindow();

let msg = $state('');
let showModal = $state(false);
// cli 인풋 데이터 -> 로컬스토리지에 저장된 정보 로드
let port_departure = $state(localStorage.getItem('port_departure_daily') || 'PortA');
let port_arrival = $state(localStorage.getItem('port_arrival_daily') || 'PortB');
let tz_arrival = $state(localStorage.getItem('tz_arrival_daily') || 'GMT+0');
let dt_arrival = $state(localStorage.getItem('dt_arrival_daily') || '');
let lng_density = $state(localStorage.getItem('lng_density_daily') || '447.093');
let bog_density = $state(localStorage.getItem('bog_density_daily') || '0.779');
let bog_lhv = $state(localStorage.getItem('bog_lhv_daily') || '45400');

// /** @type {HTMLTextAreaElement} */
// let textArea;
/** @type {MsgPanel}*/
let msgPanel;
/** @type {ExportBtn} */
let btn;

$effect(() => {
    localStorage.setItem('port_departure_daily', port_departure);
    localStorage.setItem('port_arrival_daily', port_arrival);
    localStorage.setItem('tz_arrival_daily', tz_arrival);
    localStorage.setItem('dt_arrival_daily', dt_arrival);
    localStorage.setItem('lng_density_daily', lng_density);
    localStorage.setItem('bog_density_daily', bog_density);
    localStorage.setItem('bog_lhv_daily', bog_lhv);
});

appWebView.listen('message', (event) => {
    msg = msg.concat(`${event.payload}\n`);
    // console.log(event.payload);

    // scroll down
    // textArea.scrollTop = textArea.scrollHeight;
    // msgPanel.toBottom();
});

/**
 * @param {Date} dt UTC datetime
 * @returns {string} formatted string
*/
function format(dt) {
    const year = dt.getFullYear();
    const month = String(dt.getMonth() + 1).padStart(2, '0');
    const day = String(dt.getDate()).padStart(2, '0');
    const hours = String(dt.getHours()).padStart(2, '0');
    const minutes = String(dt.getMinutes()).padStart(2, '0');
    const seconds = String(dt.getSeconds()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

async function exportDailyReport() {
    //! daily report 출력 -> cli 명령 실행
    // 리포트 출력 폴더 선택
    const output_file = await save({
        filters: [
            {
                name: 'Excel',
                extensions: ['xlsx'],
            },
        ],
        defaultPath: "C:\\Users\\H5495\\Documents\\daily_report",
    });

    if (output_file === null) {
        return;
    }

    showModal = true;

    let arrival = new Date(`${dt_arrival} 12:00:00`);
    let departure = new Date(arrival.getTime() - 24 * 60 * 60 * 1000)

    const input = YAML.stringify({
        port_departure: port_departure,
        port_arrival: port_arrival,
        tz_departure: tz_arrival,
        tz_arrival: tz_arrival,
        departure: format(departure),
        arrival: format(arrival),
        lng_density: Number(lng_density),
        bog_density: Number(bog_density),
        bog_lhv: Number(bog_lhv),
        output_file: output_file,
    });

    msg = '';
    btn.deactivateBtn();

    invoke('export_report', {input: input, repoType: "daily"})
        .then(msg => {
            console.log(msg);
        })
        .catch(err => {
            console.error(err);
            alert(err);
        })
        .finally(() => {
            btn.activateBtn();
            showModal = false;
        })
}
</script>

<article class="container">
    <h1>Daily Report</h1>

    <div class="input-panel">
        <span class="item-title">Departure Port</span>
        <span></span>
        <input type="text" bind:value={port_departure}>

        <span class="item-title">Arrival Port</span>
        <span></span>
        <input type="text" bind:value={port_arrival}>

        <span class="item-title">Datetime</span>
        <select name="tz-daily" bind:value={tz_arrival}>
            {#each gmt_list as gmt}
                <option value={gmt}>{gmt}</option>
            {/each}
        </select>
        <input name="dt-daily" type="date" bind:value={dt_arrival}>

        <span class="item-title">LNG Density</span>
        <span class="item-title">kg/m3</span>
        <div class="input-btn-area">
            <input type="number" bind:value={lng_density}>
            <button>auto</button>
        </div>

        <span class="item-title">BOG Density</span>
        <span class="item-title">kg/m3</span>
        <div class="input-btn-area">
            <input type="number" bind:value={bog_density}>
            <button>auto</button>
        </div>

        <span class="item-title">BOG LHV</span>
        <span class="item-title">kJ/kg</span>
        <div class="input-btn-area">
            <input type="number" bind:value={bog_lhv}>
            <button>auto</button>
        </div>

        <div class="export-btn-area">
            <ExportBtn bind:this={btn} action={exportDailyReport}></ExportBtn>
        </div>
    </div>

    <div style="height: 33vh; background-color: red;">
        <MsgPanel bind:this={msgPanel} bind:msg={msg}></MsgPanel>
    </div>
</article>

<Modal bind:showModal={showModal}></Modal>

<style>
.input-btn-area {
    display: flex;
    column-gap: 0.5em;
    width: 100%;
    height: 100%;
}

.input-btn-area > input {
    width: 100%;
}

.input-panel {
    display: grid;
    grid-template-columns: 0.4fr 0.5fr 0.7fr;
    row-gap: 0.5em;
    column-gap: 2em;
    padding: 1em 0;
}

.item-title {
    display: flex;
    /* text-indent: 0.1em; */
    justify-content: left;
    align-items: center;
}


.export-btn-area {
    display: flex;
    justify-content: right;
    width: 100%;
    grid-row: 9;
    grid-column: 2 / 4;
}

input {
    height: 1em;
}

button {
    padding: 0 1em;
}

select {
    border-radius: 8px;
    border: 1px solid transparent;
    /* padding: 0.6em 1.2em; */
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    outline: none;
}

article {
    box-sizing: border-box;
    width: 100%;
    padding: 1em 3vw;
}
</style>