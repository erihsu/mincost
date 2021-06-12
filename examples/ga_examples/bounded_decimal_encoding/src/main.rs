use mincost::{Evolution, EvolutionConfig, Individual};
use rand::{thread_rng, Rng};
use std::iter::repeat_with;

fn main() {
    // define randness strategy
    let randness = || -> Individual<i32> {
        let mut rng = thread_rng();
        Individual {
            genes: repeat_with(|| rng.gen_range(100..200)).take(10).collect(),
        }
    };

    // define fitness function
    let fitness = |solution: &Individual<i32>| -> f32 {
        let weight: Vec<i32> = (-5..5).collect();
        let sum: i32 = solution.genes.iter().zip(weight).fold(0, |acc, (g, w)| {
            let score = g * w;
            acc + score
        });
        sum as f32
    };
    // hyper parameter in ga
    let evolution_config = EvolutionConfig {
        pop_size: 100,
        elite_size: 20,
        mutation_rate: 0.1,
        generations: 20,
    };
    let mut evolution = Evolution::init(evolution_config, fitness, randness).unwrap();
    let best_ind = evolution.evolute().unwrap();
    println!("Best DeciIndividual {:?}", best_ind);
}
