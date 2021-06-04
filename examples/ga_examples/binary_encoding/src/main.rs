use mincost::Individual;

use mincost::Evolution;
fn main() {
    // define Fitness as well as encoding type.
    // In this example, encoding type is Boolean
    let fitness = |solution: &Individual<bool>| -> i32 {
        let weight: Vec<i32> = (-5..5).collect();
        solution.genes.iter().zip(weight).fold(0, |acc, (g, w)| {
            let score = if *g == true { w } else { -w };
            acc + score
        })
    };

    let mut evolution = Evolution::init("config.yml", fitness).unwrap();
    let best_ind = evolution.evolute().unwrap();
    println!("Best Individual {:?}", best_ind);
}
