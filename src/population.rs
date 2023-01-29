use crate::genetics::Evolution;
use crate::rocket::Rocket;
use crate::world::{Wall, World};
use crate::NUM_ROCKETS;
use nannou::Draw;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// A group of simulation agents
pub struct Population {
    /// All simulation agents (Rockets) in the current generation
    agents: Vec<Rocket>,
    /// Gene pool refers to all genes within a population
    /// A gene pool is formulated at the end of every generation
    /// and is then used to select the best genes for the rockets in the next generation
    ///
    /// `WeightedIndex` is a distribution wherein the chance of picking a given element is
    /// proportional to weight assigned to the element
    /// For more info refer:
    /// https://docs.rs/rand/latest/rand/distributions/struct.WeightedIndex.html
    gene_pool: Option<WeightedIndex<f32>>,
}

impl Population {
    pub fn new() -> Self {
        Population::initialize()
    }

    /// Update every rocket in the current generation
    pub fn update(&mut self, frame_idx: usize, world: &World) {
        for r in &mut self.agents {
            let is_wall = world.is_wall(&r.pos);
            r.update(frame_idx, is_wall);
        }
    }

    /// Draw every rocket for the current generation
    pub fn draw(&self, draw: &Draw) {
        self.agents.iter().for_each(|v| v.draw(&draw));
    }
}

/// Defines an `Evolutionary` cycle for the `Population`
/// Refer `Evolution` for a brief summary on the steps involved during evolution
///
/// Or refer the link below for more info on evolutionary algorithms,
/// https://natureofcode.com/book/chapter-9-the-evolution-of-code/
impl Evolution for Population {
    /// Create the initial set of rockets to begin the simulation
    fn initialize() -> Self {
        let rockets: Vec<Rocket> = (0..NUM_ROCKETS).map(|_| Rocket::new(None)).collect();
        Population {
            agents: rockets,
            gene_pool: None,
        }
    }

    /// Calculate the quality of the population,
    /// This can be achieved by calculating `fitness` of every rocket in the population.
    /// Fitness can be defined as a measure of how "good" the solution is
    /// wrt the problem under consideration
    fn selection(&mut self) {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        // Calculate fitness for every rocket
        for v in &mut self.agents {
            let fitness = v.fitness();
            weights.push(fitness);

            if fitness > max_fitness {
                max_fitness = fitness;
            }
        }

        // Normalize fitness,
        // i.e map the fitness to a value between 0 and 100,
        // this helps avoid values being too large or too small
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness) * 100.0);

        self.gene_pool = Some(WeightedIndex::new(&weights)
            .expect("Failed to generate gene pool"));
    }

    /// Use the results from `selection` and generate new rockets for the next generation
    fn reproduction(&mut self) {
        let mut rng = thread_rng();
        let mut new_population = Vec::new();

        match &self.gene_pool {
            Some(genes) => {
                for _ in 0..NUM_ROCKETS {
                    // Select two rockets randomly
                    // The probability of picking a rocket is proportional to its fitness
                    let first = self.agents[genes.sample(&mut rng)].clone();
                    let second = self.agents[genes.sample(&mut rng)].clone();
                    let child = Rocket::reproduce(&first, &second);
                    new_population.push(child);
                }

                self.agents.clear();
                self.agents = new_population;
            }

            // No gene pool,
            // new rockets can't be generated
            None => return,
        }
    }
}
