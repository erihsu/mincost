///! Particle Swarm Optimization Framework
use std::fmt::Debug;
use std::iter::repeat_with;

type Solution<T> = Vec<T>;

pub struct Particle<T> {
    position: Vec<T>,
    velocity: Vec<T>,
    best_known_position: Vec<T>,
}

impl<T> Particle<T> {
    fn update_position(&mut self, lr: f32, dimension: usize)
    where
        T: std::ops::AddAssign + Into<f32> + From<f32> + Copy,
    {
        for d in 0..dimension {
            self.position[d] += T::from(lr * self.velocity[d].into());
        }
    }
}

pub struct Swarm<T> {
    population: Vec<Particle<T>>,
    best_known_position: Vec<T>,
}

pub struct PsOpt<T, F> {
    swarm: Swarm<T>,
    fitness: F,
    config: PsoConfig,
}

#[derive(Debug)]
pub struct PsoConfig {
    pop_size: usize,
    omega: f32, // w
    phi_g: f32,
    phi_p: f32,
    learning_rate: f32, // lr
    iteration: usize,
}

impl<T> Swarm<T> {
    fn initial_random_pop<R: Fn() -> Particle<T>>(pop_size: usize, randness: R) -> Self {
        Swarm {
            population: repeat_with(|| randness()).take(pop_size).collect(),
            best_known_position: randness().best_known_position,
        }
    }
    fn update_swarm<O: PartialOrd>(
        &mut self,
        config: &PsoConfig,
        fitness: &dyn Fn(&Solution<T>) -> O,
    ) where
        T: std::ops::AddAssign + std::ops::Sub<Output = T> + Into<f32> + From<f32> + Copy,
    {
        let dimension = self.best_known_position.len();
        for p in self.population.iter_mut() {
            for d in 0..dimension {
                // update velocity in each dimension of particle
                let r_p = fastrand::f32();
                let r_g = fastrand::f32();
                p.velocity[d] = T::from(
                    config.omega * p.velocity[d].into()
                        + config.phi_p * r_p * (p.best_known_position[d] - p.position[d]).into()
                        + config.phi_g * r_g * (self.best_known_position[d] - p.position[d]).into(),
                );
            }
            p.update_position(config.learning_rate, dimension);
            if fitness(&p.position) < fitness(&p.best_known_position) {
                p.best_known_position = p.position.clone();
                if fitness(&p.best_known_position) < fitness(&self.best_known_position) {
                    self.best_known_position = p.best_known_position.clone();
                }
            }
        }
    }
}
use std::ops::*;
impl<T, F, O> PsOpt<T, F>
where
    F: Fn(&Solution<T>) -> O,
    O: PartialOrd,
    T: Copy + Debug + AddAssign + Sub + std::ops::Sub<Output = T> + From<f32> + Into<f32>,
{
    pub fn init<R: Fn() -> Particle<T>>(config: PsoConfig, fitness: F, randness: R) -> Self
    where
        F: Fn(&Particle<T>) -> O,
        O: Into<f64>,
    {
        let swarm = Swarm::initial_random_pop(config.pop_size, randness);
        PsOpt {
            swarm,
            fitness,
            config,
        }
    }
    pub fn optimize(&mut self) -> Solution<T> {
        // reach iteration number as termination criterion
        for _ in 0..self.config.iteration {
            self.swarm.update_swarm(&self.config, &self.fitness);
        }
        self.swarm.best_known_position.clone()
    }
}
