import * as sim from "lib-simulation-wasm";

var simulation;
function restart() {
  
  var sliderValueAnimal = document.getElementById("num-animals").value;
  var sliderValueFoods = document.getElementById("num-foods").value;
  var sliderValueFOVRange = document.getElementById("fov-range").value;
  var sliderValueFOVAngle = document.getElementById("fov-angle").value;
  var sliderValueCells = document.getElementById("cells").value;


  
  simulation = new sim.Simulation(sliderValueAnimal, sliderValueFoods, sliderValueFOVRange, sliderValueFOVAngle, sliderValueCells);
}

restart();





const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;


const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

// canvas rendering context
const ctxt = viewport.getContext('2d');

CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size, rotation) {
        this.beginPath();
        this.moveTo(
          x - Math.sin(rotation) * size * 1.5, 
          y + Math.cos(rotation) * size * 1.5,
        );

        this.lineTo(
            x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );

        this.fillStyle = 'rgb(255, 255, 255)';
        this.fill();
        this.stroke();
    };

CanvasRenderingContext2D.prototype.drawCircle =
    function(x, y, radius) {
        this.beginPath();

        this.arc(x, y, radius, 0, 2.0 * Math.PI);

        this.fillStyle = 'rgb(0, 255, 128)';
        this.fill();
    };


function redraw() {
    ctxt.clearRect(0, 0, viewportWidth + 10, viewportHeight + 10); // +10 bc of weird bug where it wouldn clear the edges 

    
    var sliderValueSpeedMin = document.getElementById("speed-min").value;
    var sliderValueSpeedMax = document.getElementById("speed-max").value;
    var sliderValueSpeedAccel = document.getElementById("speed-accel").value;
    var sliderValueRotationAccel = document.getElementById("rotation-accel").value;
    var sliderValueGenerationLength = document.getElementById("generation-length").value;
    
    var sliderValueFOVRange = document.getElementById("fov-range").value;
    var sliderValueFOVAngle = document.getElementById("fov-angle").value;
    var sliderValueCells = document.getElementById("cells").value;

    if (sliderValueSpeedMin > sliderValueSpeedMax) {
      sliderValueSpeedMin = sliderValueSpeedMax; 
      document.getElementById("speed-min").value = sliderValueSpeedMax;
    }

    simulation.step(
      sliderValueSpeedMin,
      sliderValueSpeedMax,
      sliderValueSpeedAccel,
      sliderValueRotationAccel,
      sliderValueGenerationLength,
      sliderValueFOVRange,
      sliderValueFOVAngle,
      sliderValueCells,
    );

    // console.log(simulation.get_generation())
    var spanGeneration = document.getElementById("gen-num");
    spanGeneration.textContent = simulation.get_generation();

    const world = simulation.world();

    for (const food of world.foods) {
        ctxt.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        );
    }

    // console.log(world.animals[0].rotation)
    for (const animal of world.animals) {
        ctxt.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.01 * viewportWidth,
            animal.rotation,
        );
    }

    // requestAnimationFrame() schedules code only for the next frame.
    //
    // Because we want for our simulation to continue forever, we've
    // gotta keep re-scheduling our function:
    requestAnimationFrame(redraw);
}

redraw();

