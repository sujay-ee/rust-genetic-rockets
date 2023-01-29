//! Simulation Controller

use crate::genetics::Evolution;
use crate::ROCKET_LIFESPAN;
use nannou::Draw;

use crate::population::Population;
use crate::world::World;

/// Simulation State
/// `model` for nannou application
pub struct Simulation {
    /// Maintains the current generation number
    /// or current simulation cycle number
    /// Starts with 0, incremented by 1 at the end of every generation
    pub generation_count: u32,
    /// Current frame index, updated every frame
    /// Defines how long a generation lasts
    frame_idx: usize,
    /// `Population` of rockets
    population: Population,
    /// Simulation environment i.e the `World`
    world: World,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            generation_count: 0,
            frame_idx: 0,
            population: Population::new(),
            world: match World::new() {
                Ok(w) => w,
                Err(_) => panic!("Error trying to create world"),
            },
        }
    }

    /// Update the simulation
    /// Every generation lasts for `ROCKET_LIFESPAN` frames
    /// Then a new generation begins
    pub fn update(&mut self) {
        if self.frame_idx == 0 {
            self.end_current_generation();
            self.start_new_generation();
        }

        self.frame_idx = (self.frame_idx + 1) % ROCKET_LIFESPAN;
        self.population.update(self.frame_idx, &self.world);
    }

    /// Draw the simulation to the window
    pub fn draw(&self, draw: &Draw) {
        self.world.draw(&draw);
        self.population.draw(&draw);
    }

    fn start_new_generation(&mut self) {
        self.generation_count += 1;
        self.population.reproduction();
    }

    fn end_current_generation(&mut self) {
        // There is no generation to be ended
        if self.generation_count == 0 {
            return;
        }

        self.frame_idx = 0;
        self.population.selection();
    }
}
