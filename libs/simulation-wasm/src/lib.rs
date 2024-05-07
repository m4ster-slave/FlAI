use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,

    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self { animals, foods }
    }
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(animals: i32, foods: i32, fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng, animals, foods, fov_range, fov_angle, cells);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(
        &mut self,
        speed_min: f32,
        speed_max: f32,
        speed_accel: f32,
        rotation_accel: f32,
        generation_length: i32,
        fov_range: f32,
        fov_angle: f32,
        cells: usize,
    ) {
        self.sim.step(
            &mut self.rng,
            speed_min,
            speed_max,
            speed_accel,
            rotation_accel,
            generation_length,
            fov_range,
            fov_angle,
            cells,
        );
    }

    pub fn get_generation(&mut self) -> i32 {
        self.sim.generation
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
