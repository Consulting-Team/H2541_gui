export const ssr = false;
/** @type {import('./$types').LayoutLoad} */
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
        default_tz_departure: 'GMT+0',
        default_tz_arrival: 'GMT+0',
        default_departure: default_departure,
        default_arrival: default_arrival,
        default_lng_density: 445.056,
        default_bog_density: 0.78011,
        default_bog_lhv: 49540,
    }
}
