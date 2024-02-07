import { Simulation, Cell } from "nito";
import { memory } from "nito/nito_bg";

const CELL_SIZE = 5;
let toolSize = 5;

const CELL_COLORS = [
  "rgba(255, 255, 240)", // Air
  "rgba(33, 163, 219)", // Water
  "rgba(246, 215, 176)", // Sand
  "rgba(130, 94, 51)", // Wood
  "rgba(255, 85, 33)", // Fire
  "rgba(200, 200, 200)", // Smoke
  "rgba(0, 200, 0)", // Acid
  "rgba(237, 54, 33)", // Ember
  "rgba(200, 200, 0)", // Gas
  "rgba(80, 80, 80)", // Stone
  "rgba(30, 30, 30)", // Coal
  "rgba(255, 255, 255)", // Salt
  "rgba(128, 128, 128)", // Cinder
  "rgba(255, 0, 0)", // Lava
  "rgba(126, 0, 135)", // Oil
  "rgba(0, 255, 0)", // Moss
  "rgba(255, 255, 0)", // Canon Powder
  "rgba(0, 255, 255)", // Ice
];

// Construct the universe, and get its width and height.
const universe = Simulation.new(100, 100);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE) * height;
canvas.width = (CELL_SIZE) * width;


const ctx = canvas.getContext('2d');

let animationId = null;

const renderLoop = () => {

  drawGrid();
  drawCells();

  universe.update();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const slider = document.getElementById('slider');
const output = document.getElementById('output-tooltip');

// Display the initial value
output.textContent = slider.value + '%';

// Update the output whenever the slider value changes
slider.addEventListener('input', function() {
    output.textContent = this.value + '%';
});

const material = document.getElementsByClassName("material");
let materialIndex = 0;
material[0].style.border = "2px solid black";
material[0].style.padding = "8px";
for (let i = 0; i < material.length; i++) {
  material[i].addEventListener("click", event => {
    // universe.set_material(i);
    if (materialIndex != i) {
      material[materialIndex].style.border = "none";
      material[materialIndex].style.padding = "10px";
      material[i].style.border = "2px solid black";
      material[i].style.padding = "8px";
      materialIndex = i;
    }
  }
)}


const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = "transparent";

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.dump();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      ctx.fillStyle = CELL_COLORS[cells[idx]];
      ctx.fillRect(
        col * (CELL_SIZE),
        row * (CELL_SIZE),
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
};

const squareDistance = (x1, y1, x2, y2) => {
  return Math.pow(x1 - x2, 2) + Math.pow(y1 - y2, 2);
}

let drawing = false;

const handleMouseEvent = (event) => {
  if (!drawing) return;
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE)), height);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE)), width);

  // universe.toggle_cell(row, col);
  //toggle_cell in area of toolSize and center in row, col
  for (let i = 0; i < toolSize; i++) {
    for (let j = 0; j < toolSize; j++) {
      let x = row - Math.floor(toolSize / 2) + i;
      let y = col - Math.floor(toolSize / 2) + j;
      if (x < 0 || x >= height || y < 0 || y >= width) continue;
      if (squareDistance(row, col, x, y) > Math.pow(toolSize / 2, 2)) continue;
      universe.set_cell(y, x, materialIndex + 1);
      console.log (x, y);
    }
  }

  drawCells();
  drawGrid();
};

const customMouse = document.getElementById('custom-mouse');
const body = document.getElementById('body');

canvas.addEventListener("mousedown", (event) => { drawing = true; handleMouseEvent(event); });
canvas.addEventListener("mouseup", (event) => { drawing = false; handleMouseEvent(event);});
canvas.addEventListener("mousemove", handleMouseEvent);
canvas.addEventListener("mouseleave", (event) => { drawing = false; customMouse.style.border = "none";body.style.cursor = "default"; });
canvas.addEventListener("mouseenter", (event) => { drawing = false; customMouse.style.border = "2px solid black"; body.style.cursor = "none"; });
canvas.addEventListener("wheel", (event) => {
  toolSize = Math.min(10, Math.max(1, toolSize - Math.sign(event.deltaY) / 3));
  customMouse.style.width = toolSize * CELL_SIZE + 'px';
  customMouse.style.height = toolSize * CELL_SIZE + 'px';
  customMouse.style.left = event.pageX - (CELL_SIZE * 2) - (toolSize * CELL_SIZE) / 2 + 'px';
  customMouse.style.top = event.pageY - (CELL_SIZE * 2) - (toolSize * CELL_SIZE) / 2 + 'px';
});


// Update the position of the custom mouse based on mouse movements
document.addEventListener('mousemove', (event) => {
  customMouse.style.left = event.pageX - (CELL_SIZE * 2) - (toolSize * CELL_SIZE) / 2 + 'px';
  customMouse.style.top = event.pageY - (CELL_SIZE * 2) - (toolSize * CELL_SIZE) / 2 + 'px';
});

play();
