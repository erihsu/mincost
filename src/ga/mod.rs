#![allow(dead_code)]
use self::error::GaError;
/// Generic Algorithm in Rust
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::Distribution;

use rand::{thread_rng, Rng};
use std::fmt::Debug;

pub type GaResult<T> = std::result::Result<T, error::GaError>;

#[derive(Clone, Debug)]
pub struct Individual<T> {
    pub genes: Vec<T>,
}

pub struct Evolution<T, F> {
    config: EvolutionConfig<T>,
    population: Population<T>,
    fitness: F,
}

#[derive(Debug, PartialEq)]
pub struct EvolutionConfig<T> {
    pub pop_size: usize,
    pub elite_size: usize,
    pub mutation_rate: f32,
    pub generations: usize,
    pub individual_length: usize,
    pub upper: Option<T>,
    pub lower: Option<T>,
}

impl<T> Individual<T>
where
    T: Copy + Debug,
{
    fn breed(&self, another: &Self) -> Self {
        let mut rng = thread_rng();
        let idx1: usize = rng.gen_range(0..self.genes.len());
        let idx2: usize = rng.gen_range(0..self.genes.len());
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
    // self-mutated
    fn mutate(&mut self) {
        let mut rng = thread_rng();
        let idx1: usize = rng.gen_range(0..self.genes.len());
        let idx2: usize = rng.gen_range(0..self.genes.len());
        // swap gene within chromo
        let tmp = self.genes[idx1];
        self.genes[idx1] = self.genes[idx2];
        self.genes[idx2] = tmp;
    }
}

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
    T: Copy + Debug,
{
    pub fn init(config: EvolutionConfig<T>, fitness: F) -> GaResult<Self>
    where
        T: PartialOrd,
        Standard: Distribution<T>,
    {
        let population =
            Population::initial_without_range(config.pop_size, config.individual_length);
        Ok(Evolution {
            config,
            population,
            fitness,
        })
    }
    pub fn init_with_range(config: EvolutionConfig<T>, fitness: F) -> GaResult<Self>
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let population = Population::initial_with_range(
            config.pop_size,
            config.individual_length,
            config.lower.unwrap(),
            config.upper.unwrap(),
        );
        Ok(Evolution {
            config,
            population,
            fitness,
        })
    }
    fn next_generation(&mut self) -> GaResult<Population<T>> {
        self.population.rank(&self.fitness);
        let mut breeded = self
            .population
            .selection(&self.config, &self.fitness)
            .and_then(|mut x| x.breed(&self.config))?;
        let _ = breeded.mutate(&self.config)?;
        Ok(breeded)
    }
    pub fn evolute(&mut self) -> GaResult<Individual<T>> {
        for _ in 0..self.config.generations {
            let next_gen: Population<T> = self.next_generation()?;
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
    T: Copy + Debug,
{
    fn initial_with_range(pop_size: usize, length: usize, lower: T, upper: T) -> Self
    where
        T: SampleUniform + PartialOrd,
        Standard: Distribution<T>,
    {
        let mut rng = thread_rng();
        Population {
            individuals: repeat_with(|| Individual {
                genes: repeat_with(|| rng.gen_range(lower..upper))
                    .take(length)
                    .collect(),
            })
            .take(pop_size)
            .collect(),
            status: PopulationStatus::Initialized,
        }
    }
    fn initial_without_range(pop_size: usize, length: usize) -> Self
    where
        Standard: Distribution<T>,
    {
        Population {
            individuals: repeat_with(|| Individual {
                genes: repeat_with(|| rand::random::<T>()).take(length).collect(),
            })
            .take(pop_size)
            .collect(),
            status: PopulationStatus::Initialized,
        }
    }
    // inplace rank between individuals by fitness
    fn rank<O: PartialOrd>(&mut self, fitness: &dyn Fn(&Individual<T>) -> O)
    where
        O: PartialOrd,
    {
        self.individuals
            .sort_by(|a, b| fitness(a).partial_cmp(&fitness(b)).unwrap());
        self.status = PopulationStatus::Ranked;
    }
    fn selection<O: PartialOrd>(
        &mut self,
        config: &EvolutionConfig<T>,
        fitness: &dyn Fn(&Individual<T>) -> O,
    ) -> GaResult<Self>
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

    fn breed(&mut self, config: &EvolutionConfig<T>) -> GaResult<Self> {
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

    fn mutate(&mut self, config: &EvolutionConfig<T>) -> GaResult<()> {
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

    fn best_individual(&self) -> GaResult<Individual<T>> {
        if self.status == PopulationStatus::Mutated {
            Ok(self.individuals[0].clone())
        } else {
            Err(GaError::BestDeciIndividualNotReady)
        }
    }
}
mod error;
