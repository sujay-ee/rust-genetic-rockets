//! Rocket - The Simulation Agent

use crate::{
    ROCKET_COLOR, ROCKET_COLOR_COMPLETED, ROCKET_COLOR_CRASHED, ROCKET_SIZE, ROCKET_SPAWN_LOCATION,
    ROCKET_STROKE_COLOR, TARGET_LOCATION, TARGET_RADIUS,
};
use nannou::glam::{vec2, Vec2};
use nannou::prelude::*;
use nannou::Draw;

use crate::genetics::{Dna, Reproduction};

#[derive(Clone)]
pub enum RocketState {
    Alive,     // Can move around
    Crashed,   // Collided with a wall
    Completed, // Reached target
}

/// Rocket
/// The autonomous agent capable of moving around the world
/// https://en.wikipedia.org/wiki/Autonomous_agent
///
/// Forces govern how the rocket moves in a 2D plain
/// Refer the below link for more info on how the 3 vectors,
/// `pos`, `vel` and `acc` interact with one another
/// https://natureofcode.com/book/chapter-2-forces/
#[derive(Clone)]
pub struct Rocket {
    /// The world position of the rocket
    pub pos: Vec2,
    /// Velocity
    /// A vector representing the speed and direction of motion
    vel: Vec2,
    /// Acceleration
    /// Force applied to the rocket every frame
    /// Or rate of change of velocity per frame
    acc: Vec2,
    /// Current state of the rocket
    state: RocketState,
    /// Genetic information related to the rocket
    dna: Dna,
}

impl Rocket {
    pub fn new(dna: Option<&Dna>) -> Self {
        Self {
            pos: vec2(ROCKET_SPAWN_LOCATION.0, ROCKET_SPAWN_LOCATION.1),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            state: RocketState::Alive,
            // Use the `dna` if provided, else randomize it
            dna: dna.unwrap_or(&Dna::new(None)).clone(),
        }
    }

    pub fn update(&mut self, frame_idx: usize, is_wall: bool) {
        if let RocketState::Crashed | RocketState::Completed = self.state {
            return;
        }

        // Target collision
        if self.target_distance() <= TARGET_RADIUS {
            self.state = RocketState::Completed;
            return;
        }

        // Wall or boundary collision
        if is_wall {
            self.state = RocketState::Crashed;
            return;
        }

        // Update the position of the rocket based on its velocity
        self.apply_force(&self.dna.get(frame_idx));
    }

    pub fn draw(&self, draw: &Draw) {
        // Calculate the direction the rocket must face
        let theta = self.vel.angle() + PI / 2.0;
        draw.rect()
            .color(self.color())
            .w_h(ROCKET_SIZE.0, ROCKET_SIZE.1)
            .x_y(self.pos.x, self.pos.y)
            .stroke(ROCKET_STROKE_COLOR)
            .stroke_weight(0.5)
            .rotate(theta);
    }

    /// Create a new child rocket from `first` and `second` parent rockets
    pub fn reproduce(first: &Self, second: &Self) -> Self {
        let child_dna = Dna::crossover(&first.dna, &second.dna).mutate();
        Rocket::new(Some(&child_dna))
    }

    /// The fitness function
    /// https://en.wikipedia.org/wiki/Fitness_function
    ///
    /// We follow a simple approach here,
    /// Fitness is inversely proportional to distance to the target
    /// i.e the closer the rocket is to the target, the larger is its fitness
    ///
    /// This approach doesn't yield accurate results for a complex world with walls
    /// close to the target, since collisions with walls
    /// near target will have a high fitness
    /// One approach to fix this is to use a Flood fill algorithm
    pub fn fitness(&mut self) -> f32 {
        // Division by zero leads to `inf`
        let inv_target_dist = 1.0 / self.target_distance();

        // Optional step, amplifies the fitness value
        inv_target_dist * inv_target_dist
    }

    /// Return the distance between the rocket and the target
    fn target_distance(&self) -> f32 {
        self.pos
            .distance(vec2(TARGET_LOCATION.0, TARGET_LOCATION.1))
    }

    /// Return a color based on the rocket state
    fn color(&self) -> Rgb<u8> {
        match self.state {
            RocketState::Crashed => ROCKET_COLOR_CRASHED,
            RocketState::Completed => ROCKET_COLOR_COMPLETED,
            _ => ROCKET_COLOR,
        }
    }

    /// Update rocket position by updating its acceleration
    /// Refer https://natureofcode.com/book/chapter-2-forces/
    /// for more details on the internal working
    fn apply_force(&mut self, force: &Vec2) {
        self.acc += *force;
        self.vel += self.acc;
        self.pos += self.vel;
        self.acc = Vec2::ZERO;
    }
}
