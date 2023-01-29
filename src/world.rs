//! Simulation environment

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::{
    GRID_COLOR, MAP_FILE_PATH, SCREEN_DIMENSIONS, TARGET_COLOR, TARGET_LOCATION, TARGET_RADIUS,
};
use nannou::prelude::*;
use nannou::Draw;

/// The environment & settings to which the simulation agents are constrained to
pub struct World {
    /// Regions in the world rockets can't pass through
    /// This doesn't include the window boundaries
    /// The walls are defined in the map file `MAP_FILE_PATH`
    walls: Vec<(usize, usize)>,
    /// Number of rows/columns in the grid the world is mapped to
    /// This is defined in the map file and is assumed to be a square
    grid_size: usize,
    /// Size of a unit block which represents either wall/no-wall
    block_size: f32,
}

/// A wall is a solid block that a rocket can't pass through
pub trait Wall<T> {
    /// Check if a wall exists exists at `pos`
    fn is_wall(&self, pos: T) -> bool;
}

impl World {
    pub fn new() -> io::Result<World> {
        let (grid_size, walls) = World::load_map_data()?;
        let block_size = SCREEN_DIMENSIONS as f32 / (grid_size as f32);
        Ok(World {
            walls,
            grid_size,
            block_size,
        })
    }

    /// Render the world
    pub fn draw(&self, draw: &Draw) {
        let half_screen = (SCREEN_DIMENSIONS as f32) * 0.5;
        let half_block_size = self.block_size * 0.5;

        // Draw the wall blocks
        // This is called on every simulation draw cycle
        // and hence not scalable for large grids
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                if self.is_wall((i, j)) != true {
                    continue;
                }

                let x = -half_screen + (self.block_size * j as f32);
                let y = half_screen - (self.block_size * i as f32);
                draw.rect()
                    .x_y(x + half_block_size, y - half_block_size)
                    .w_h(self.block_size, self.block_size)
                    .color(GRID_COLOR);
            }
        }

        // Draw the target
        draw.ellipse()
            .x_y(TARGET_LOCATION.0, TARGET_LOCATION.1)
            .w_h(TARGET_RADIUS, TARGET_RADIUS)
            .color(TARGET_COLOR);
    }

    /// Map a window position `pos` to grid position `(x, y)`
    fn window_to_grid(pos: &Vec2) -> (usize, usize) {
        let half_screen = (SCREEN_DIMENSIONS as f32) * 0.5;
        let x = pos.x - -half_screen;
        let y = pos.y - half_screen;
        let x = (x / 24.0).abs() as usize;
        let y = (y / 24.0).abs() as usize;

        (x, y)
    }

    /// Parse the input grid file located at `MAP_FILE_PATH`
    /// And create a vector of all wall positions
    fn load_map_data() -> io::Result<(usize, Vec<(usize, usize)>)> {
        // IO errors propagated to caller
        // TODO: handle what happens with the map file isn't a square
        let file = File::open(MAP_FILE_PATH)?;
        let reader = BufReader::new(file);
        let mut grid = Vec::new();
        let mut grid_size = 0;

        for (i, line) in reader.lines().into_iter().enumerate() {
            if let Ok(line) = line {
                // TODO is there a better way to calc grid matrix size?
                grid_size = line.len();
                for (j, c) in line.chars().enumerate() {
                    if c == '1' {
                        continue;
                    }

                    grid.push((i, j));
                }
            }
        }

        Ok((grid_size, grid))
    }
}

impl Wall<(usize, usize)> for World {
    /// Return `true` if the a wall exists at `(x, y)`,
    /// `false` otherwise
    ///
    /// Here `x` and `y` have to be grid co-ordinates
    fn is_wall(&self, (x, y): (usize, usize)) -> bool {
        self.walls.contains(&(x, y))
    }
}

impl Wall<&Vec2> for World {
    /// Check if a wall exists in a given world position `pos`
    fn is_wall(&self, pos: &Vec2) -> bool {
        let half_screen = (SCREEN_DIMENSIONS as f32) * 0.5;

        // Window boundaries are walls
        let left_right = pos.x <= -half_screen || pos.x >= half_screen;
        let top_down = pos.y <= -half_screen || pos.y >= half_screen;
        if left_right || top_down {
            return true;
        }

        // Actual walls from map matrix
        let (x, y) = World::window_to_grid(&pos);
        self.is_wall((y, x)) == true
    }
}
