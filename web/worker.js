import init, { solve } from "./rust_scripts/puzzle_a_day.js";

let wasmReady = false;

init().then(() => {
  wasmReady = true;
});

self.onmessage = async (event) => {
  const { month, day, board } = event.data;

  if (!wasmReady) {
    alert("WASM module not ready");
    return;
  }

  const result = solve(month, day, board);
  self.postMessage({ result });
};
