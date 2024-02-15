import { Simulation } from "nito";
import { memory } from "nito/nito_bg";

let CELL_SIZE = 5;
const WIDTH = 200;
const HEIGHT = 200;
let toolSize = 5;

const CELL_COLORS = [
  [255, 255, 240],  // Air
  [33, 163, 219],   // Water
  [246, 215, 176],  // Sand
  [130, 94, 51],    // Wood
  [255, 85, 33],    // Fire
  [200, 200, 200],  // Smoke
  [0, 200, 0],      // Acid
  [237, 54, 33],    // Ember
  [200, 200, 0],    // Gas
  [80, 80, 80],     // Stone
  [30, 30, 30],     // Coal
  [255, 255, 255],  // Salt
  [128, 128, 128],  // Cinder
  [255, 0, 0],      // Lava
  [126, 0, 135],    // Oil
  [0, 255, 0],      // Moss
  [255, 255, 0],    // Canon Powder
  [0, 255, 255],    // Ice
];

const universe = Simulation.new(WIDTH, HEIGHT);
const slider = document.getElementById('slider');
const output = document.getElementById('output-tooltip');

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("nito-canvas");
canvas.height = (CELL_SIZE) * HEIGHT;
canvas.width = (CELL_SIZE) * WIDTH;


const ctx = canvas.getContext('2d');

let drawing = false;
let paused = false;

let mouseX = 0;
let mouseY = 0;

const updateMousePosition = (event) => {
  mouseX = event.clientX;
  mouseY = event.clientY;
}


let lastTimestamp = 0;


const renderLoop = (timestamp) => {
  if (drawing) {
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    const canvasLeft = (mouseX - boundingRect.left) * scaleX;
    const canvasTop = (mouseY - boundingRect.top) * scaleY;
    const row = Math.min(Math.round(canvasTop / (CELL_SIZE)), HEIGHT);
    const col = Math.min(Math.round(canvasLeft / (CELL_SIZE)), WIDTH);
    for (let i = 0; i < toolSize; i++) {
      for (let j = 0; j < toolSize; j++) {
        let x = row - Math.round(toolSize / 2) + i;
        let y = col - Math.round(toolSize / 2) + j;
        if (x < 0 || x >= HEIGHT || y < 0 || y >= WIDTH) continue;
        if (squareDistance(row, col, x, y) > Math.pow(toolSize / 2, 2)) continue;
        universe.set_cell(y, x, materialIndex);
      }
    }
  }
  drawCells(false);

  const elapsedFrameTime = timestamp - lastTimestamp;
  const delay = Math.max(0, 1000 / (slider.value * 60 / 100) - elapsedFrameTime);
  if (delay <= 0 && paused == false) {
    universe.update();
    lastTimestamp = timestamp;
  }
  requestAnimationFrame(renderLoop);
};

// on window resize
window.addEventListener('resize', () => {
  CELL_SIZE = Math.max(1, Math.min(10, Math.floor(window.innerHeight / HEIGHT)));
  canvas.height = (CELL_SIZE) * HEIGHT;
  canvas.width = (CELL_SIZE) * WIDTH;
  drawCells(true);
});

const squareDistance = (x1, y1, x2, y2) => {
  return Math.pow(x1 - x2, 2) + Math.pow(y1 - y2, 2);
}




const playPauseButton = document.getElementById("play-pause");
const clearButton = document.getElementById("clear");

clearButton.addEventListener("click", () => {
  universe.clear();
  drawCells(false);
});

const play = () => {
  playPauseButton.textContent = "⏸";
  paused = false;
};

const pause = () => {
  playPauseButton.textContent = "▶";
  paused = true;
};

playPauseButton.addEventListener("click", event => {
  if (paused) {
    play();
  } else {
    pause();
  }
});



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

const getIndex = (row, column) => {
  return row * WIDTH + column;
};

const getCellColor = (type, variant) => {
  let color = CELL_COLORS[type];
  let d;
  switch (type) {
    case 0:
      return ('rgb(' + color[0] + ',' + color[1] + ',' + color[2] + ')');
    default:
      let variantT = variant * 5 / 255;
      d = ((variantT / 5) * 0.2 + 0.8);
      return ('rgb(' + color[0] * d + ',' + color[1] * d + ',' + color[2] * d + ')');
  }
};


const drawCells = (reset) => {
  const cellsPtr = universe.dump();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (WIDTH * HEIGHT) * 3);
  ctx.beginPath();

  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      if (cells[getIndex(row, col) * 3 + 2] == 0 && reset == false) continue;
      const idx = getIndex(row, col);
      ctx.fillStyle = getCellColor(cells[idx * 3], cells[idx * 3 + 1]);
      ctx.fillRect(
        col * (CELL_SIZE),
        row * (CELL_SIZE),
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
};


const customMouse = document.getElementById('custom-mouse');
const body = document.getElementById('body');

canvas.addEventListener("mousedown", () => {drawing = true;});
canvas.addEventListener("mouseup", () => {drawing = false;});
canvas.addEventListener('mousemove', updateMousePosition);
canvas.addEventListener("mouseleave", () => { drawing = false; customMouse.style.border = "none";body.style.cursor = "default"; });
canvas.addEventListener("mouseenter", () => { drawing = false; customMouse.style.border = "2px solid black"; body.style.cursor = "none"; });
canvas.addEventListener("wheel", (event) => {
  toolSize = Math.min(10, Math.max(1.1, toolSize - Math.sign(event.deltaY) / 3));
  customMouse.style.width = toolSize * CELL_SIZE + 'px';
  customMouse.style.height = toolSize * CELL_SIZE + 'px';
  customMouse.style.left = event.pageX - (toolSize * CELL_SIZE) / 2 + 'px';
  customMouse.style.top = event.pageY - (toolSize * CELL_SIZE) / 2 + 'px';
});


// Update the position of the custom mouse based on mouse movements
document.addEventListener('mousemove', (event) => {
  customMouse.style.width = toolSize * CELL_SIZE + 'px';
  customMouse.style.height = toolSize * CELL_SIZE + 'px';
  customMouse.style.left = event.pageX -  (toolSize * CELL_SIZE) / 2 + 'px';
  customMouse.style.top = event.pageY -  (toolSize * CELL_SIZE) / 2 + 'px';
});

requestAnimationFrame(renderLoop);
