import init, { solve, get_board_options } from "./rust_scripts/puzzle_a_day.js";

let wasmReady = false;
init().then(() => {
  wasmReady = true;
});

let PIECES = [];
const COLORS = [
  "#FF5733",
  "#33FF57",
  "#3357FF",
  "#F333FF",
  "#33FFF5",
  "#F5FF33",
  "#FF33A8",
  "#A833FF",
  "#33FFA8",
  "#FFA833",
  "#8D33FF",
  "#33D1FF",
];

const worker = new Worker("./worker.js", { type: "module" });
worker.onmessage = (event) => {
  const { result, error } = event.data;

  if (error) {
    console.error("Error solving puzzle:", error);
    alert(error);
  } else {
    // console.log("Solve result:", result);
    PIECES = result;
    update_solution();
  }
};

export const update_solution = () => {
  const solutionDiv = document.getElementById("solution");
  solutionDiv.innerHTML = "";

  if (PIECES.length <= 0) {
    return;
  }

  const hidePieces = parseInt(document.getElementById("hide-pieces").value, 10);

  // find boundary rectangle
  const max_x = Math.max(
    ...PIECES.map(({ coordinates }) =>
      Math.max(...coordinates.map(({ x }) => x))
    )
  );
  const max_y = Math.max(
    ...PIECES.map(({ coordinates }) =>
      Math.max(...coordinates.map(({ y }) => y))
    )
  );

  // build a map of piece index -> color
  const colorMap = {};
  for (let i = 0; i < PIECES.length; i++) {
    colorMap[i] = COLORS[i % COLORS.length];
  }

  // then filter pieces based on hidePieces
  const visiblePieces = PIECES.slice(
    0,
    Math.max(PIECES.length - hidePieces, 0)
  );

  // define the display grid
  const displayGrid = [];
  for (let y = 0; y <= max_y; y++) {
    displayGrid[y] = [];
    for (let x = 0; x <= max_x; x++) {
      displayGrid[y][x] = "#0000"; // default transparent
    }
  }

  // populate the displayGrid with piece colors
  for (let i = 0; i < visiblePieces.length; i++) {
    for (let coor of visiblePieces[i].coordinates) {
      const { x, y } = coor;
      displayGrid[y][x] = colorMap[i];
    }
  }

  // then grab screen size, do some calculations (we need square cells, max width of entire board = min(screen width, 800px))
  const screenWidth = window.innerWidth;
  const maxBoardWidth = Math.min(screenWidth - 40, 800); // 40px padding
  const cellSize = Math.floor(maxBoardWidth / (max_x + 1));

  // then update the html
  let html = `<div style="display: grid; grid-template-columns: repeat(${
    max_x + 1
  }, ${cellSize}px); grid-template-rows: repeat(${
    max_y + 1
  }, ${cellSize}px); gap: 1px;">`;
  for (let y = 0; y <= max_y; y++) {
    for (let x = 0; x <= max_x; x++) {
      const color = displayGrid[y][x];
      html += `<div style="width: ${cellSize}px; height: ${cellSize}px; background-color: ${color};"></div>`;
    }
  }
  html += `</div>`;

  solutionDiv.innerHTML = html;
};

export const trigger_solve = () => {
  const dateElem = document.getElementById("target-time");
  const boardElem = document.getElementById("board-select");
  console.log("Triggering solve for date:", dateElem.value);
  const monthMatch = dateElem.value.match(/-(\d{2})-/);
  const dayMatch = dateElem.value.match(/-(\d{2})T/);
  worker.postMessage({
    month: monthMatch[1],
    day: dayMatch[1],
    board: boardElem.value,
  });
};

export const update_pieces_hidden = () => {
  const value = document.getElementById("hide-pieces").value;
  const text = `Hide ${value} Pieces`;
  document.getElementById("hide-pieces-label").innerText = text;
  update_solution();
};

const populate_board_options = async () => {
  if (!wasmReady) {
    await init();
    wasmReady = true;
  }

  const options = get_board_options();
  const selectElement = document.getElementById("board-select");
  options.forEach((option) => {
    const opt = document.createElement("option");
    opt.value = option;
    opt.innerHTML = option;
    selectElement.appendChild(opt);
  });
};

const set_default_date_time = () => {
  const now = new Date();
  const offsetMs = now.getTimezoneOffset() * 60000;
  const localDate = new Date(now - offsetMs);
  const formattedDatetime = localDate.toISOString().slice(0, 16);
  document.getElementById("target-time").value = formattedDatetime;
};

window.onload = function () {
  set_default_date_time();
  update_pieces_hidden();
  populate_board_options();
};

window.trigger_solve = trigger_solve;
window.update_pieces_hidden = update_pieces_hidden;
