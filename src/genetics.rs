//! Genetics of the simulated agent
//!
//! The genetic data necessary for functioning, reproduction
//! and evolution of a simulation agent.
//!
//! Refer the following link for more theory related to evolution and genetic algorithm,
//! https://natureofcode.com/book/chapter-9-the-evolution-of-code/

use crate::{MUTATION_PROBABILITY, MUTATION_VARIATION, ROCKET_LIFESPAN};
use nannou::glam::{vec2, Vec2};
use nannou::prelude::*;

/// Reproduction defines a strategy to create a new agent given one/more parent agents
pub trait Reproduction {
    /// Crossover involves creating a new child given the genetic code of two parents
    fn crossover(first: &Self, second: &Self) -> Self;
    /// An optional step that involves performing minor variations on a crossed over child,
    /// defined by a mutation-rate
    fn mutate(&self) -> Self;
}

/// Evolution is the mechanism by which an agent can accumulate change
/// in its characteristics progressively over generations
pub trait Evolution {
    /// Create a population of N agents,
    /// Each with randomly generated `DNA`,
    /// This as generation zero
    fn initialize() -> Self;
    /// Rank the population of agents to
    /// calculate how well they performed in the current generation
    fn selection(&mut self);
    /// Choose the best performing agents in the current generation
    /// to be parents for the next generation
    fn reproduction(&mut self);
}

/// DNA - The genetic data encoded into a simulation agent
#[derive(Clone)]
pub struct Dna {
    /// Genes - A list of vectors defining the motion trajectory of an agent
    genes: Vec<Vec2>,
}

impl Dna {
    /// Creates a new DNA
    ///
    /// If `genes` argument is `None`, gene vectors are randomized
    /// Else creates a DNA with `genes`
    pub fn new(genes: Option<&Vec<Vec2>>) -> Self {
        if let Some(genes) = genes {
            return Dna {
                genes: genes.clone(),
            };
        }

        let genes: Vec<Vec2> = (0..ROCKET_LIFESPAN)
            .map(|_| vec2(
                random_range(-1.0, 1.0),
                random_range(-1.0, 1.0)))
            .collect();
        Dna { genes }
    }

    /// Retrieve the gene vector at `index`
    /// Panics if `index` is invalid
    pub fn get(&self, index: usize) -> Vec2 {
        *self
            .genes
            .get(index as usize)
            .expect("gene index out of rocket lifespan bound")
    }
}

impl Reproduction for Dna {
    /// Returns the DNA of the child using `first` and `second` as parents
    ///
    /// Strategy:
    /// 1. Pick a random mid-point
    /// 2. Then take genes from index 0 to mid-point (exclusive) from the `first`
    /// 3. And genes from mid-point to end-of-vector from `second`
    fn crossover(first: &Self, second: &Self) -> Self {
        let split_point = random_range(0, first.genes.len());

        let mut new_genes = Vec::from_iter(
            first.genes[0..split_point].iter().cloned());
        new_genes.extend_from_slice(&second.genes[split_point..]);

        Dna::new(Some(&new_genes))
    }

    /// Returns an altered gene data based on mutation rate
    /// defined by `MUTATION_PROBABILITY`
    fn mutate(&self) -> Self {
        let mut mutated_genes = self.genes.clone();
        for g in &mut mutated_genes {
            if random_f32() > (MUTATION_PROBABILITY as f32) * 0.001 {
                continue;
            }

            g.x += random_range(-1.0, 1.0) * MUTATION_VARIATION;
            g.y += random_range(-1.0, 1.0) * MUTATION_VARIATION;
        }

        Dna::new(Some(&mutated_genes))
    }
}
