https://pwy.io/posts/learning-to-fly-pt4/
https://pwy.io/posts/learning-to-fly-pt3/#the-test


 NODE_OPTIONS=--openssl-legacy-provider npm run start




#[derive(Clone, Debug)]
pub struct Config {
    pub speed_min: f32,
    pub speed_max: f32,
    pub speed_accel: f32,
    pub rotation_accel: f32,
    pub generation_length: u32,
    pub fov_range: f32,
    pub fov_angle: f32,
    pub cells: usize,
    pub num_foods: u32,
    pub num_animals: u32,
    pub num_gen: u32,
    pub top_animal: u32,
}

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;
const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;
