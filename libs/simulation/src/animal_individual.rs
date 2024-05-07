use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(), 
        }
        
    }

    pub fn into_animal(self, rng: &mut dyn RngCore, fov_range: f32, fov_angle: f32, cells: usize) -> Animal {
        Animal::from_chromosome(self.chromosome, rng, fov_range, fov_angle, cells)
    }
}
