use mincost::{Annealer, AnnealerConfig, Solution};
use rand::prelude::SliceRandom;
use rand::thread_rng;
fn main() {
    // say if you want to travel 8 cities in China: 0.Beijing 1.Shanghai 2.Hangzhou 3.Wuhan 4.Chengdu 5.Nanjing
    // 6.Chongqing 7.Guangzhou

    // distance matrix of 8 cities(in km), source from https://www.distancecalculator.net
    let dis_matrix = [
        [0, 1213, 1120, 1160, 1516, 896, 1456, 1885],
        [1213, 0, 161, 687, 1655, 268, 1434, 1206],
        [1120, 161, 0, 566, 1542, 239, 1309, 1045],
        [1160, 687, 566, 0, 978, 459, 754, 836],
        [1516, 1655, 1542, 978, 0, 1404, 269, 1238],
        [896, 268, 239, 239, 459, 0, 1199, 1131],
        [1456, 1434, 1309, 754, 269, 1199, 0, 979],
        [1885, 1206, 1045, 836, 1238, 1131, 979, 0],
    ];
    // give hyper parameter in ga
    let evolution_config = AnnealerConfig {
        alpha: 0.8,
        temperature_zero: 80.0,
        temperature_end: 5.0,
        iteration: 10,
    };
    // define randness strategy
    let randness = || -> Solution<usize> {
        let mut rng = thread_rng();
        let mut index = vec![0, 1, 2, 3, 4, 5, 6, 7];
        index.shuffle(&mut rng);
        Solution { bits: index }
    };
    // define fitness function
    let fitness = |solution: &Solution<usize>| -> i32 {
        (0..7).fold(0, |acc, idx| {
            let i = solution.bits[idx];
            let j = solution.bits[idx + 1];

            acc + dis_matrix[i][j]
        })
    };
    // construct annuler
    let mut evolution = Annealer::init(evolution_config, fitness, randness);
    let best_ind = evolution.anneal();
    println!("Total Path Length :{:?} km", fitness(&best_ind));
    println!(
        "Best Travel Route: {:?}",
        best_ind
            .bits
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" -> ")
    );
}
