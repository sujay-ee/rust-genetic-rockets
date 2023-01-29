use nannou::prelude::*;

// Window Configs
pub const SCREEN_DIMENSIONS: u32 = 720;

// Simulation Configs
pub const NUM_ROCKETS: u32 = 750;
pub const SIM_BACKGROUND: Rgb<u8> = DARKSLATEGREY;

// Target Configs
pub const TARGET_LOCATION: (f32, f32) = (330.0, 0.0);
pub const TARGET_COLOR: Rgb<u8> = GOLD;
pub const TARGET_RADIUS: f32 = 22.0;

// Grid Configs
pub const GRID_COLOR: Rgb<u8> = PALEVIOLETRED;
pub const MAP_FILE_PATH: &str = "assets/map.txt";

// Rocket Configs
pub const ROCKET_LIFESPAN: usize = 200;
pub const ROCKET_SPAWN_LOCATION: (f32, f32) = (-350.0, 0.0);
pub const ROCKET_SIZE: (f32, f32) = (5.0, 20.0);
pub const ROCKET_COLOR: Rgb<u8> = WHITE;
pub const ROCKET_STROKE_COLOR: Rgb<u8> = DIMGREY;
pub const ROCKET_COLOR_COMPLETED: Rgb<u8> = GREENYELLOW;
pub const ROCKET_COLOR_CRASHED: Rgb<u8> = LIGHTSLATEGREY;

// Mutation Configs
pub const MUTATION_PROBABILITY: u8 = 10;
pub const MUTATION_VARIATION: f32 = 0.5;
