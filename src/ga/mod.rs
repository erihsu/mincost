#![allow(dead_code)]
/// Generic Algorithm in Rust
use crate::error::GaError;
use crate::GaResult;
use std::fmt::Debug;
pub trait Individual: Clone + Debug {
    // generate valid rand individual
    fn rand() -> Self;
    // fitness
    fn fitness(&self) -> i32;
    // breed with another Individual
    fn breed(&self, another: &Self) -> Self;
    // self-mutated
    fn mutate(&mut self);
}

#[derive(PartialEq)]
pub enum PopulationStatus {
    Initialized,
    Ranked,
    Selected,
    Breeded,
    Mutated,
}

pub struct Evolution<I> {
    config: EvolutionConfig,
    population: Population<I>,
}

use std::fs;
use std::path::Path;

impl<I> Evolution<I>
where
    I: Individual,
{
    pub fn init<P: AsRef<Path>>(config_path: P) -> GaResult<Self> {
        let cfg_str: String = fs::read_to_string(config_path)?;
        let config: EvolutionConfig = serde_yaml::from_str(&cfg_str)?;
        let population = Population::initial(&config);
        Ok(Evolution { config, population })
    }
    fn next_generation(&mut self) -> GaResult<Population<I>> {
        self.population.rank();
        let mut breeded = self
            .population
            .selection(&self.config)
            .and_then(|mut x| x.breed(&self.config))?;
        let _ = breeded.mutate(&self.config)?;
        Ok(breeded)
    }
    pub fn evolute(&mut self) -> GaResult<I> {
        for _ in 0..self.config.generations {
            let next_gen: Population<I> = self.next_generation()?;
            self.population = next_gen;
        }
        self.population.best_individual()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pop_size: usize,
    elite_size: usize,
    mutation_rate: f32,
    generations: usize,
}

pub struct Population<I> {
    individuals: Vec<I>,
    status: PopulationStatus,
}

impl<I> Population<I>
where
    I: Individual,
{
    fn initial(config: &EvolutionConfig) -> Self {
        let mut individuals = Vec::with_capacity(config.pop_size);
        for _ in 0..config.pop_size {
            individuals.push(I::rand());
        }
        Population {
            individuals,
            status: PopulationStatus::Initialized,
        }
    }
    // inplace rank between individuals by fitness
    fn rank(&mut self) {
        self.individuals
            .sort_by(|a, b| a.fitness().cmp(&b.fitness()));
        self.status = PopulationStatus::Ranked;
    }
    fn selection(&mut self, config: &EvolutionConfig) -> GaResult<Self> {
        let mut selected = Vec::with_capacity(config.pop_size);
        let mut tmp: i32 = 0;
        let fitness: Vec<i32> = self.individuals.iter().map(|x| x.fitness()).collect();
        let fitness_sum: i32 = fitness.iter().fold(0, |acc, x| acc + x);
        let cum_perc: Vec<i32> = fitness
            .iter()
            .map(|x| {
                tmp += x;
                (100 * tmp / fitness_sum) as i32
            })
            .collect();
        if self.status == PopulationStatus::Ranked {
            // keep elite from last generation
            for i in 0..config.elite_size {
                selected.push(self.individuals[i].clone());
            }
            // select high score individuals to form the complete generation
            for _ in 0..config.pop_size - config.elite_size {
                let pick = 100 * rand::random::<f32>() as i32;
                for j in 0..config.pop_size {
                    if pick <= cum_perc[j] {
                        selected.push(self.individuals[j].clone());
                        break;
                    }
                }
            }
            Ok(Population {
                individuals: selected,
                status: PopulationStatus::Selected,
            })
        } else {
            Err(GaError::SelectionBeforeRank)
        }
    }

    fn breed(&mut self, config: &EvolutionConfig) -> GaResult<Self> {
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
                child.push(p1.breed(&p2));
            }
            Ok(Population {
                individuals: child,
                status: PopulationStatus::Breeded,
            })
        } else {
            Err(GaError::BreedBeforeSelection)
        }
    }

    fn mutate(&mut self, config: &EvolutionConfig) -> GaResult<()> {
        if self.status == PopulationStatus::Breeded {
            for ind in self.individuals.iter_mut() {
                if rand::random::<f32>() < config.mutation_rate {
                    ind.mutate();
                }
            }
            self.status = PopulationStatus::Mutated;
            Ok(())
        } else {
            Err(GaError::MutateBeforeBreed)
        }
    }

    fn best_individual(&self) -> GaResult<I> {
        if self.status == PopulationStatus::Mutated {
            Ok(self.individuals[0].clone())
        } else {
            Err(GaError::BestIndividualNotReady)
        }
    }
}

pub type GaResult<T> = std::result::Result<T, error::GaError>;
