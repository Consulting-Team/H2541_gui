<script>
    import { invoke } from "@tauri-apps/api/core";
    import { save } from '@tauri-apps/plugin-dialog';
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import YAML from 'yaml';
    import ExportBtn from '$lib/export_btn.svelte';

    /** @type {import('./$types').PageProps}*/
    const { data } = $props();
    const appWebView = getCurrentWebviewWindow();

    let msg = $state('');
    // cli 인풋 데이터 -> 로컬스토리지에 저장된 정보 로드
    let port_departure = $state(localStorage.getItem('port_departure') || '');
    let port_arrival = $state(localStorage.getItem('port_arrival') || '');
    let tz_departure = $state(localStorage.getItem('tz_departure') || 'GMT+0');
    let tz_arrival = $state(localStorage.getItem('tz_arrival') || 'GMT+0');
    let dt_departure = $state(localStorage.getItem('dt_departure') || '');
    let dt_arrival = $state(localStorage.getItem('dt_arrival') || '');
    let lng_density = $state(localStorage.getItem('lng_density') || '');
    let bog_density = $state(localStorage.getItem('bog_density') || '');
    let bog_lhv = $state(localStorage.getItem('bog_lhv') || '');

    $effect(() => {
        localStorage.setItem('port_departure', port_departure);
        localStorage.setItem('port_arrival', port_arrival);
        localStorage.setItem('tz_departure', tz_departure);
        localStorage.setItem('tz_arrival', tz_arrival);
        localStorage.setItem('dt_departure', dt_departure);
        localStorage.setItem('dt_arrival', dt_arrival);
        localStorage.setItem('lng_density', lng_density);
        localStorage.setItem('bog_density', bog_density);
        localStorage.setItem('bog_lhv', bog_lhv);
    });

    /** @type {HTMLTextAreaElement} */
    let textArea;

    appWebView.listen('message', (event) => {
        msg = msg.concat(`\n${event.payload}`);
        textArea.scrollTop = textArea.scrollHeight;
    });

    async function export_voyage_report() {

        // 리포트 출력 폴더 선택
        const output_file = await save({
            filters: [
                {
                    name: 'Excel',
                    extensions: ['xlsx'],
                },
            ],
            defaultPath: "C:\\Users\\H5495\\Documents",
        });

        // report_gen 입력
        const input = YAML.stringify({
            port_departure: port_departure,
            port_arrival: port_arrival,
            tz_departure: tz_departure,
            tz_arrival: tz_arrival,
            departure: dt_departure,
            arrival: dt_arrival,
            lng_density: lng_density,
            bog_density: bog_density,
            bog_lhv: bog_lhv,
            output_file: output_file,
        });

        let out = invoke('export_voyage_report', {input: input});
        // out.then(res => {
        //     console.log(res);
        //     msg = res;
        // });

        // console.log(input);
    }

</script>

 <article class="container">
    <h1>Voyage Report</h1>

    <div class=input-panel>
        <span class="item-title">Departure port</span>
        <span></span>
        <input type="text" bind:value={port_departure}>

        <span class="item-title">Arrival port</span>
        <span></span>
        <input type="text" bind:value={port_arrival}>

        <label for="dt-departure" class="item-title">Departure</label>
        <select name="tz-departure" bind:value={tz_departure}>
            {#each data.gmt_list as gmt}
                <option value={gmt}>{gmt}</option>
            {/each}
        </select>
        <input name="dt-departure" type="datetime" bind:value={dt_departure}>

        <label for="dt-arrival" class="item-title">Arrival</label>
        <select name="tz-arrival" bind:value={tz_arrival}>
            {#each data.gmt_list as gmt}
                <option value={gmt}>{gmt}</option>
            {/each}
        </select>
        <input name="tz-arrival" type="datetime" bind:value={dt_arrival}>

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
            <ExportBtn action={ export_voyage_report }></ExportBtn>
        </div>
    </div>

    <textarea bind:this={textArea} bind:value={msg}></textarea>
 </article>

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
    padding: 1em;
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

textarea {
    border: 1px solid black;
    /* width: calc(100% - 2em); */
    width: 100%;
    box-sizing: border-box;
    height: 20vh;
    font-family: 'Courier New', Courier, monospace;
    overflow-x: scroll;
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
    padding: 1em 3em;
}

</style>