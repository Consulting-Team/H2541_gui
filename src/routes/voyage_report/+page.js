/** @type {import('./$types').PageLoad} */
export function load() {
    const default_arrival = new Date().toISOString().slice(0, 16).replace("T", " ") + ":00";
    const default_departure = (() => {
        const d = new Date(default_arrival);
        d.setUTCDate(d.getUTCDate() - 7);
        return d.toISOString().slice(0, 16).replace("T", " ") + ":00";
    })();

    return {
        default_port_departure: 'Port A',
        default_port_arrival: 'Port B',
        default_arrival: default_arrival,
        default_tz_departure: 'GMT+0',
        default_tz_arrival: 'GMT+0',
        default_departure: default_departure,
        default_lng_density: 445.056,
        default_bog_density: 0.78011,
        default_bog_lhv: 49540,
        gmt_list: [
            "GMT-12",
            "GMT-11",
            "GMT-10",
            "GMT-9",
            "GMT-8",
            "GMT-7",
            "GMT-6",
            "GMT-5",
            "GMT-4",
            "GMT-3",
            "GMT-2",
            "GMT-1",
            "GMT+0",
            "GMT+1",
            "GMT+2",
            "GMT+3",
            "GMT+4",
            "GMT+5",
            "GMT+6",
            "GMT+7",
            "GMT+8",
            "GMT+9",
            "GMT+10",
            "GMT+11",
            "GMT+12",
        ]
    }
}