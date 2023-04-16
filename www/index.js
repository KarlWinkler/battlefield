import { Universe } from "wasm-game-of-life";

let universe = Universe.new(26, 13);
universe.tick();