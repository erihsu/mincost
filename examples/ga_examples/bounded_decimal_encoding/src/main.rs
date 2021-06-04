use mincost::Evolution;
use mincost::Individual;

fn main() {
    let fitness = |solution: &Individual<i32>| -> f32 {
        let weight: Vec<i32> = (-5..5).collect();
        let sum: i32 = solution.genes.iter().zip(weight).fold(0, |acc, (g, w)| {
            let score = g * w;
            acc + score
        });
        sum as f32
    };

    let mut evolution = Evolution::init_with_range("config.yml", fitness).unwrap();
    let best_ind = evolution.evolute().unwrap();
    println!("Best DeciIndividual {:?}", best_ind);
}
