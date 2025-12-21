import init, { solve } from "./rust_scripts/puzzle_a_day.js";

let wasmReady = false;

init().then(() => {
  wasmReady = true;
});

self.onmessage = async (event) => {
  const { month, day, week_day, custom_board } = event.data;

  if (!wasmReady) {
    alert("WASM module not ready");
    return;
  }

  try {
    const result = solve(month, day, week_day, custom_board);
    self.postMessage({ result });
  } catch (error) {
    self.postMessage({ error: error.message });
  }
};
