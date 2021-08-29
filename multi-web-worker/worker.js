// First message is init
self.onmessage = async event => {
  let [module, memory, tx] = event.data;

  let { default: init, entry } = await import('./pkg/wasm_hello_world.js')
  await init(module, memory);
  entry(tx);

  // We don't expect any further messages
  self.onmessage = () => {
    throw new Error("Unexpected");
  }
}
