import init, { Pool, WorkerPool } from "/wasm/web.js";

async function main() {
  await init("/wasm/web.wasm");

  let worker = new WorkerPool(navigator.hardwareConcurrency);
  let pool = new Pool();
  // pool.render(worker);
}
main();
