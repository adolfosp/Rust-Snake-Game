import init, { World } from "snake_game";

init().then((_) => {
  const CELL_SIZE = 10;
  const world = World.new();
  const worldWidth = world.width();
  const canvas = document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

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
  
  function drawSnake() {
   const snakeIdx = world.snake_head_idx();
   const col = snakeIdx % worldWidth;
   const row = Math.floor(snakeIdx / worldWidth);

   ctx.beginPath();
   ctx.fillRect(
    col * CELL_SIZE,// x
    row * CELL_SIZE,// y
    CELL_SIZE,// width
    CELL_SIZE// height
   );
   ctx.stroke();
  }

  drawWorld();
  drawSnake();

  setInterval(() => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawWorld();
    drawSnake();
    world.update();
  }, 1000); 
});
