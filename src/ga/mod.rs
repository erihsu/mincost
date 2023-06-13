//! Genetic Algorithm Framework
//!
//! Four steps to construct generic algorithm
//!
//! 1. Give hyper parameter in GA
//! ``` rust
//! let config = EvolutionConfig {...}
//! ```

//! 2. Define individual encoding and its randomization
//! ``` rust
//! use std::iter::repeat_with;
//! let randness = || -> Individual<bool> {
//!     Individual {
//!         genes: repeat_with(|| rand::random::<bool>()).take(10).collect(),
//!     }
//! };
//! ```
//! 3. Define fitness function by closure
//! ``` rust
//! let fitness = |solution: &Individual<bool>| -> f32 {
//!     ...
//! };
//! ```
//! 4. Construct genetic algorithm
//! ``` rust
//! let mut evolution = Evolution::init(config, fitness, randness);
//! ```

//! Finally, run the process to get the optimized solution
//! ``` rust
//! let best_ind = evolution.evolute();
//! ```

//! Learn more from the [examples](examples/ga_examples)
use std::fmt::Debug;

/// generic individual to support various encoding style
#[derive(Clone, Debug)]
pub struct Individual<T> {
    /// genes in the individual
    pub genes: Vec<T>,
}

/// evolution body
pub struct Evolution<T, F> {
    config: EvolutionConfig,
    population: Population<T>,
    fitness: F,
}

/// hyper parameter in genetic algorithm
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EvolutionConfig {
    /// population size
    pub pop_size: usize,
    /// elite size
    pub elite_size: usize,
    /// mutattion rate, in 0 to 1
    pub mutation_rate: f32,
    /// evolution generation number
    pub generations: usize,
}

impl<T> Individual<T>
where
    T: Copy + Debug + std::cmp::PartialEq,
{
    // breed1, default breed method.
    fn breed1(&self, another: &Self) -> Self {
        let idx1: usize = fastrand::usize(..self.genes.len());
        let idx2: usize = fastrand::usize(..self.genes.len());
        let start_gene_idx = std::cmp::min(idx1, idx2);
        let end_gene_idx = std::cmp::max(idx1, idx2);
        let child_p1 = &self.genes[start_gene_idx..end_gene_idx];
        let mut child_p2 = [
            &another.genes[0..start_gene_idx],
            &another.genes[end_gene_idx..],
        ]
        .concat(); // slice concating
        child_p2.extend(child_p1);
        Individual { genes: child_p2 }
    }
    // breed2, special breed method. It ensures the child's gene bit is non-duplicated with each other
    fn breed2(&self, another: &Self) -> Self {
        let idx1: usize = fastrand::usize(..self.genes.len());
        let idx2: usize = fastrand::usize(..self.genes.len());
        let start_gene_idx = std::cmp::min(idx1, idx2);
        let end_gene_idx = std::cmp::max(idx1, idx2);
        let child_p1 = &self.genes[start_gene_idx..end_gene_idx];
        let mut child_p2 = vec![];

        for g in &another.genes {
            if !child_p1.contains(g) {
                child_p2.push(*g);
            }
        }
        child_p2.extend(child_p1);
        assert_eq!(child_p2.len(), self.genes.len());
        Individual { genes: child_p2 }
    }
    // self-mutated
    fn mutate(&mut self) {
        let idx1: usize = fastrand::usize(..self.genes.len());
        let idx2: usize = fastrand::usize(..self.genes.len());
        // swap gene within chromo
        let tmp = self.genes[idx1];
        self.genes[idx1] = self.genes[idx2];
        self.genes[idx2] = tmp;
    }
}

// internal state of population evolution
#[derive(PartialEq)]
enum PopulationStatus {
    Initialized,
    Ranked,
    Selected,
    Breeded,
    Mutated,
}

impl<T, F, O> Evolution<T, F>
where
    F: Fn(&Individual<T>) -> O,
    O: PartialOrd + Into<f64>,
    T: Copy + Debug + std::cmp::PartialEq,
{
    /// initial envolution, including population and evolution hyper parameter
    pub fn init<R: Fn() -> Individual<T>>(
        config: EvolutionConfig,
        fitness: F,
        randness: R,
    ) -> Self {
        let population = Population::initial_random_pop(config.pop_size, randness);
        Evolution {
            config,
            population,
            fitness,
        }
    }
    // generate next iteration
    fn next_generation(&mut self) -> Population<T> {
        self.population.rank(&self.fitness);
        let mut selected = self.population.selection(&self.config, &self.fitness);
        let mut breeded = selected.breed(&self.config);
        breeded.mutate(&self.config);
        breeded
    }
    // the top evolution
    pub fn evolute(&mut self) -> Individual<T> {
        for _ in 0..self.config.generations {
            let next_gen: Population<T> = self.next_generation();
            self.population = next_gen;
        }
        self.population.best_individual()
    }
}

struct Population<T> {
    individuals: Vec<Individual<T>>,
    status: PopulationStatus,
}

use std::iter::repeat_with;

impl<T> Population<T>
where
    T: Copy + Debug + std::cmp::PartialEq,
{
    // initial random population
    fn initial_random_pop<R: Fn() -> Individual<T>>(pop_size: usize, randness: R) -> Self {
        Population {
            individuals: repeat_with(|| randness()).take(pop_size).collect(),
            status: PopulationStatus::Initialized,
        }
    }
    // inplace rank between individuals by fitness
    fn rank<O: PartialOrd>(&mut self, fitness: &dyn Fn(&Individual<T>) -> O) {
        self.individuals
            .sort_by(|a, b| fitness(a).partial_cmp(&fitness(b)).unwrap());
        self.status = PopulationStatus::Ranked;
    }
    // individual selection within popultion
    fn selection<O: PartialOrd>(
        &mut self,
        config: &EvolutionConfig,
        fitness: &dyn Fn(&Individual<T>) -> O,
    ) -> Self
    where
        O: PartialOrd + Into<f64>,
    {
        let mut selected = Vec::with_capacity(config.pop_size);
        let mut tmp: f64 = 0.0;
        let fitness: Vec<f64> = self.individuals.iter().map(|x| fitness(x).into()).collect();
        let fitness_sum: f64 = fitness.iter().fold(0.0, |acc, x| acc + x);
        let cum_perc: Vec<i32> = fitness
            .iter()
            .map(|x| {
                tmp += x;
                (100.0 * tmp / fitness_sum) as i32
            })
            .collect();
        if self.status == PopulationStatus::Ranked {
            // keep elite from last generation
            for i in 0..config.elite_size {
                selected.push(self.individuals[i].clone());
            }
            // select high score individuals to form the complete generation
            for _ in 0..config.pop_size - config.elite_size {
                let pick = (100.0 * fastrand::f32()) as i32;
                for j in 0..config.pop_size {
                    if pick <= cum_perc[j] {
                        selected.push(self.individuals[j].clone());
                        break;
                    }
                }
            }
            Population {
                individuals: selected,
                status: PopulationStatus::Selected,
            }
        } else {
            unreachable!()
        }
    }
    // individual breed within population
    fn breed(&mut self, config: &EvolutionConfig) -> Self {
        let mut child = Vec::with_capacity(config.pop_size);
        if self.status == PopulationStatus::Selected {
            // keep elite from selected result
            for i in 0..config.elite_size {
                child.push(self.individuals[i].clone());
            }

            for i in 0..config.pop_size - config.elite_size {
                // parents
                let p1 = &self.individuals[i];
                let p2 = &self.individuals[config.pop_size - i - 1];
                if cfg!(feature = "breed2") {
                    child.push(p1.breed2(&p2));
                } else if cfg!(feature = "breed1") {
                    child.push(p1.breed1(&p2));
                }
            }
            Population {
                individuals: child,
                status: PopulationStatus::Breeded,
            }
        } else {
            unreachable!()
        }
    }
    // mutation within population
    fn mutate(&mut self, config: &EvolutionConfig) {
        if self.status == PopulationStatus::Breeded {
            for ind in self.individuals.iter_mut() {
                if fastrand::f32() < config.mutation_rate {
                    ind.mutate();
                }
            }
            self.status = PopulationStatus::Mutated;
        } else {
            unreachable!()
        }
    }
    // choose the best individual from population
    fn best_individual(&self) -> Individual<T> {
        if self.status == PopulationStatus::Mutated {
            self.individuals[0].clone()
        } else {
            unreachable!()
        }
    }
}
