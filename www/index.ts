import init, { World, Direction } from "snake_game";
import {rnd} from './utils/rnd'

init().then((wasm) => {
  const CELL_SIZE = 10;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();

  const gameControlBtn = document.getElementById("game-control-btn");

  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gameControlBtn?.addEventListener("click", _ => {
    const gameStatus = world.game_status();

    if(gameStatus === undefined){
      gameControlBtn.textContent = "Playing....";
      world.start_game();
      play();
    }else{
      location.reload();
    }
  })

  document.addEventListener("keydown", (event) => {
    switch (event.key) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;
      case "ArrowRight":
        world.change_snake_dir(Direction.Right);
        break;
      case "ArrowDown":
        world.change_snake_dir(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(x * CELL_SIZE, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, y * CELL_SIZE);
      ctx.lineTo(worldWidth * CELL_SIZE, y * CELL_SIZE);
    }

    ctx.stroke();
  }

  function drawReward() {

    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);


    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(
        col * CELL_SIZE, // x
        row * CELL_SIZE, // y
        CELL_SIZE, // width
        CELL_SIZE // height
      );
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(wasm.memory.buffer, world.snake_cells(), world.snake_length());

    snakeCells.forEach((cellIdx, i) => {

      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);

      ctx.fillStyle = i === 0 ? "#7878db" : "#000f00";
      ctx.beginPath();
      ctx.fillRect(
        col * CELL_SIZE, // x
        row * CELL_SIZE, // y
        CELL_SIZE, // width
        CELL_SIZE // height
      );
    });


    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
  }

  function play() {
    const fps = 3;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      // the method takes a callback function as an argument
      //O requestAnimationFrame ajusta automaticamente a taxa de atualização para coincidir com a taxa de frames do monitor (geralmente 60 FPS, ou seja, ~16,67ms por frame).
      //Já o setInterval executa chamadas em um intervalo fixo, sem considerar a taxa de atualização da tela, o que pode levar a frames perdidos ou inconsistências.
      requestAnimationFrame(play);
    }, 1000 / fps);
  }

  paint();
});
