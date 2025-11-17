export function getCurrentDatetime() {
    const current = new Date();
    const past = new Date(current.getTime() + 60 * 60 * 1000);

    return {
        past: current.toISOString().slice(0, 19).replace('T', ' '),
        current: past.toISOString().slice(0, 19).replace('T', ' ')
    };
}