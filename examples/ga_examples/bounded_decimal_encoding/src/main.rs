use mincost::{Evolution, EvolutionConfig, Individual};

fn main() {
    let fitness = |solution: &Individual<i32>| -> f32 {
        let weight: Vec<i32> = (-5..5).collect();
        let sum: i32 = solution.genes.iter().zip(weight).fold(0, |acc, (g, w)| {
            let score = g * w;
            acc + score
        });
        sum as f32
    };
    let evolution_config = EvolutionConfig {
        pop_size: 100,
        elite_size: 20,
        mutation_rate: 0.1,
        generations: 20,
        individual_length: 10,
        upper: Some(100),
        lower: Some(200),
    };
    let mut evolution = Evolution::init_with_range(evolution_config, fitness).unwrap();
    let best_ind = evolution.evolute().unwrap();
    println!("Best DeciIndividual {:?}", best_ind);
}
