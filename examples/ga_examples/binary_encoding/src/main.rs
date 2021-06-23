use mincost::{Evolution, EvolutionConfig, Individual};
use std::iter::repeat_with;
fn main() {
    // define Fitness as well as encoding type.
    // In this example, encoding type is Boolean
    // define randness strategy
    let randness = || -> Individual<bool> {
        Individual {
            genes: repeat_with(|| rand::random::<bool>()).take(10).collect(),
        }
    };

    let fitness = |solution: &Individual<bool>| -> i32 {
        let weight: Vec<i32> = (-5..5).collect();
        solution.genes.iter().zip(weight).fold(0, |acc, (g, w)| {
            let score = if *g == true { w } else { -w };
            acc + score
        })
    };
    let evolution_config = EvolutionConfig {
        pop_size: 100,
        elite_size: 20,
        mutation_rate: 0.4,
        generations: 10,
    };
    let mut evolution = Evolution::init(evolution_config, fitness, randness);
    let best_ind = evolution.evolute();
    println!("Best Individual {:?}", best_ind);
}
