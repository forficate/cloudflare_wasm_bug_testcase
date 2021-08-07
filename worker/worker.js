addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const { execute } = wasm_bindgen;
const instance = wasm_bindgen(wasm)

async function handleRequest(request) {
    await instance;
    const resp = await execute(request)
    return resp
}
