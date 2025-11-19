/** @type {import('./$types').PageLoad} */
export function load({ params }) {
    console.log(params.type);

    return {
        type: params.type
    }
}