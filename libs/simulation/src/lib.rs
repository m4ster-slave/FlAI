mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

use self::animal_individual::*;
pub use self::{animal::*, brain::*, eye::*, food::*, world::*};
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::vec;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: i32,
    pub generation: i32,
}

impl Simulation {
    pub fn random(
        rng: &mut dyn RngCore,
        animals: i32,
        foods: i32,
        fov_range: f32,
        fov_angle: f32,
        cells: usize,
    ) -> Self {
        let world = World::random(rng, animals, foods, fov_range, fov_angle, cells);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self {
            world,
            ga,
            age: 0,
            generation: 0,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(
        &mut self,
        rng: &mut dyn RngCore,
        speed_min: f32,
        speed_max: f32,
        speed_accel: f32,
        rotation_accel: f32,
        generation_length: i32,
        fov_range: f32,
        fov_angle: f32,
        cells: usize,
    ) {
        self.process_collisions(rng);
        self.process_brains(speed_min, speed_max, speed_accel, rotation_accel);
        self.process_movements();

        self.age += 1;

        if self.age > generation_length {
            self.generation += 1;
            self.evolve(rng, fov_range, fov_angle, cells);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore, fov_range: f32, fov_angle: f32, cells: usize) {
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
            .map(|individual| individual.into_animal(rng, fov_range, fov_angle, cells))
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

    fn process_brains(
        &mut self,
        speed_min: f32,
        speed_max: f32,
        speed_accel: f32,
        rotation_accel: f32,
    ) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);

            let response = animal.brain.nn.propagate(vision);
            // ---
            // | Limits number to given range.
            // -------------------- v---v
            let speed = response[0].clamp(-speed_accel, speed_accel);
            let rotation = response[1].clamp(-rotation_accel, rotation_accel);

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

            animal.speed = (animal.speed + speed).clamp(speed_min, speed_max);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);

            // (btw, there is no need for ROTATION_MIN or ROTATION_MAX,
            // because rotation automatically wraps from 2*PI back to 0 -
            // we've already witnessed that when we were testing eyes,
            // inside `fn rotations { ... }`.)
        }
    }
}
