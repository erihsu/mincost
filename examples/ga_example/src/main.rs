use mincost::ga::Individual;
use rand::{thread_rng, Rng};

const CHROMO_LENGTH: usize = 10;

#[derive(Debug, Clone)]
struct BinaryEncodingChromo {
    // bits of genes in the chromo
    genes: Vec<bool>,
}

impl Individual for BinaryEncodingChromo {
    fn rand() -> Self {
        let mut genes = Vec::new();
        for _ in 0..CHROMO_LENGTH {
            genes.push(rand::random::<bool>());
        }
        BinaryEncodingChromo { genes }
    }
    // fitness
    fn fitness(&self) -> i32 {
        let weight: Vec<i32> = (-5..5).collect();
        self.genes.iter().zip(weight).fold(0, |acc, (g, w)| {
            let score = if *g == true { w } else { 0 };
            acc + score
        })
    }
    // breed with another Individual
    fn breed(&self, another: &Self) -> Self {
        let mut rng = thread_rng();
        let idx1: usize = rng.gen_range(0..CHROMO_LENGTH);
        let idx2: usize = rng.gen_range(0..CHROMO_LENGTH);
        let start_gene_idx = std::cmp::min(idx1, idx2);
        let end_gene_idx = std::cmp::max(idx1, idx2);
        let child_p1 = &self.genes[start_gene_idx..end_gene_idx];
        let mut child_p2 = [
            &another.genes[0..start_gene_idx],
            &another.genes[end_gene_idx..],
        ]
        .concat(); // slice concating
        child_p2.extend(child_p1);
        BinaryEncodingChromo { genes: child_p2 }
    }
    // self-mutated
    fn mutate(&mut self) {
        let mut rng = thread_rng();
        let idx1: usize = rng.gen_range(0..CHROMO_LENGTH);
        let idx2: usize = rng.gen_range(0..CHROMO_LENGTH);
        // swap gene within chromo
        let tmp = self.genes[idx1];
        self.genes[idx1] = self.genes[idx2];
        self.genes[idx2] = tmp;
    }
}

use mincost::ga::Evolution;
fn main() {
    let mut evolution = Evolution::<BinaryEncodingChromo>::init("config.yml").unwrap();
    let best_ind: BinaryEncodingChromo = evolution.evolute().unwrap();
    println!("Best Individual {:?}", best_ind);
}
