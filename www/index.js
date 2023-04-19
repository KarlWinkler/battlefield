import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

let universe = Universe.new(26, 13);
universe.tick();

console.log(universe.height());

const canvas = document.getElementById("testing-canvas");

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();
  
  const x = ((event.clientX - boundingRect.left) / boundingRect.width) * 2 - 1;
  const y = ((event.clientY - boundingRect.top) / boundingRect.height) * -2 + 1;

  console.log(universe.hit(x, y));
  console.log(universe.height());

  universe.tick();
});
