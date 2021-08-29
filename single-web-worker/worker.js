import init, { greet } from './pkg/wasm_hello_world.js';

self.onmessage = async event => {
  await init();
  greet(event.data);
}
