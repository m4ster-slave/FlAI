mod animal;
mod animal_individual;
mod food;
mod world;
mod eye;
mod brain;

use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use nalgebra as na;
pub use self::{animal::*, brain::*, eye::*, food::*, world::*};
use rand::{Rng, RngCore};
use std::{f32::consts::FRAC_PI_2, vec};
use self::animal_individual::*;


const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

const GENERATION_LENGTH: usize = 2500;


pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world 
    }

    // pub fn step(&mut self, rng: &mut dyn RngCore) {
    //     
    // }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Step 1: Prepare birdies to be sent into the genetic algorithm
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Step 2: Evolve birdies
        let evolved_population = self.ga.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();       

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position); 

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        } 
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            // let mut rng = rand::thread_rng();
            // let y: f32 = rng.gen_range(-0.1..0.1); 
            // let x: f32 = rng.gen_range(-0.1..0.1); 
            // let response: Vec<f32> = vec![y,x]; 

            let response = animal.brain.nn.propagate(vision);
            // ---
            // | Limits number to given range.
            // -------------------- v---v
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // Our speed & rotation here are *relative* - that is: when
            // they are equal to zero, what the brain says is "keep
            // flying as you are now", not "stop flying".
            //
            // Both values being relative is crucial, because our bird's
            // brain doesn't know its own speed and rotation*, meaning
            // that it fundamentally cannot return absolute values.
            //
            // * they'd have to be provided as separate inputs to the
            //   neural network, which would make the evolution process
            //   waaay longer, if even possible.

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);

            // (btw, there is no need for ROTATION_MIN or ROTATION_MAX,
            // because rotation automatically wraps from 2*PI back to 0 -
            // we've already witnessed that when we were testing eyes,
            // inside `fn rotations { ... }`.)
        }
    }
}

