let state = "";
let wasm;

function set_state(level, board) {
  state = wasm.get_level(level, board);
}

function drawBoard() {
  const boardDiv = document.getElementById("board");
  boardDiv.innerHTML = "<code>" + state.split("|").join("\n<br>") + "</code>";
  
}
function move(direction) {
  state = wasm.move_player(state, direction);
  drawBoard();
  if (wasm.win_state(state)) {
    alert("Winner!")
  }
}

function set_state(level, board) {
  state = wasm.get_level(level, board);
  drawBoard();
}

import("./node_modules/sokoban-wasm/sokoban_wasm.js").then((js) => {
  wasm = js;
  state = wasm.get_level(1, 1);
  const boardDiv = document.getElementById("board");
  boardDiv.innerHTML = "<code>" + state.split("|").join("\n<br>") + "</code>";
 
  document.onkeydown = function(evt) {
    evt = evt || window.event;
    if (evt.key == "w" || evt.key === "ArrowUp") {
        move("U");
    }
    if (evt.key === "a" || evt.key === "ArrowLeft") {
        move("L");
    }
    if (evt.key === "s" || evt.key === "ArrowDown") {
        move("D");
    }
    if (evt.key === "d" || evt.key === "ArrowRight") {
        move("R");
    }
  };
  const levelSelect = document.getElementById("levelSelect");
  const boardSelect = document.getElementById("boardSelect");
  levelSelect.onchange = function(evt) {
    set_state(levelSelect.value, boardSelect.value);
  }
  boardSelect.onchange = function(evt) {
    set_state(levelSelect.value, boardSelect.value);
  }
  const resetButton = document.getElementById("resetBoard");
  resetButton.onclick = function(evt) {
    set_state(levelSelect.value, boardSelect.value);
  }
});
