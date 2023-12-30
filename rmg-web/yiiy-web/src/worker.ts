import init, { startup } from "/wasm/yiiy.js";

async function main() {
  await init("/wasm/yiiy.wasm");

  startup();
}

main();
