//! Nannou main,
//!
//! Nannou is a framework for creative coding in Rust,
//! A nannou program mainly consists of the following components,
//! - `model`: Initializes the model that represents the app state
//! - `view`: Draw the `model` to the window
//! - `update`: Called every frame before `view`, used to update the `model`
//!
//! More on Nannou in the link below,
//! https://guide.nannou.cc/tutorials/basics/anatomy-of-a-nannou-app.html

use nannou::prelude::*;
use genetic_rockets::Simulation;
use genetic_rockets::{SCREEN_DIMENSIONS, SIM_BACKGROUND};

/// `setup` is the nannou `model` function
/// Sets up the app state model which is `Simulation`
fn setup(app: &App) -> Simulation {
    let _window = app
        .new_window()
        .view(view)
        .build()
        .expect("Failed to setup a new window");
    Simulation::new()
}

/// Update app state
fn update(_app: &App, simulation: &mut Simulation, _: Update) {
    simulation.update();
}

/// Render the simulation on the window
fn view(app: &App, simulation: &Simulation, frame: Frame) {
    let screen = app.main_window().rect();
    let draw = app.draw();

    // use frame.nth() == 0 for single background draw
    draw.background().color(SIM_BACKGROUND);

    // Draw simulation
    simulation.draw(&draw);

    // Display FPS and generation
    let fps = format!("FPS: {}", app.fps().round());
    let generation = format!("GEN: {}", simulation.generation_count);
    draw.text(fps.as_str()).x_y(
        screen.bottom_right().x - 30.0,
        screen.bottom_right().y + 30.0,
    );
    draw.text(generation.as_str()).x_y(
        screen.bottom_right().x - 30.0,
        screen.bottom_right().y + 15.0,
    );

    draw.to_frame(app, &frame).unwrap();
}

/// Entry point,
/// Starts the simulation
fn main() {
    nannou::app(setup)
        .size(SCREEN_DIMENSIONS, SCREEN_DIMENSIONS)
        .update(update)
        .run();
}
