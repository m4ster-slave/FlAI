use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(
        rng: &mut dyn RngCore,
        animals: i32,
        foods: i32,
        fov_range: f32,
        fov_angle: f32,
        cells: usize,
    ) -> Self {
        let animals = (0..animals).map(|_| Animal::random(rng, fov_range, fov_angle, cells)).collect();

        let foods = (0..foods).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}
