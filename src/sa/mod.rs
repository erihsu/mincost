///! Simulated Annealing Framework
use std::fmt::Debug;

// encoded solution
#[derive(Clone, Debug)]
pub struct Solution<T> {
    bits: Vec<T>,
}

pub struct AnnealState<T> {
    solution: Solution<T>,
    temperature: f32,
}

#[derive(Debug, PartialEq)]
pub struct AnnealerConfig {
    alpha: f32,
    temperature_zero: f32,
    temperature_end: f32,
    iteration: usize,
}

pub struct Annealer<T, F> {
    config: AnnealerConfig,
    state: AnnealState<T>,
    fitness: F,
}

impl<T> Solution<T>
where
    T: Copy,
{
    fn neighbor(&self) -> Self {
        let mut new_solution = self.clone();
        let idx1: usize = fastrand::usize(..new_solution.bits.len());
        let idx2: usize = fastrand::usize(..new_solution.bits.len());
        let tmp = &mut new_solution.bits[idx1].clone();
        new_solution.bits[idx1] = new_solution.bits[idx2].clone();
        new_solution.bits[idx2] = *tmp;
        new_solution
    }
}
use std::ops::*;
impl<T> AnnealState<T>
where
    T: Copy,
{
    fn initial_random_state<R: Fn() -> Solution<T>>(randness: R, temp0: f32) -> Self {
        AnnealState {
            solution: randness(),
            temperature: temp0,
        }
    }
    fn update_temperature(&mut self, alpha: f32) {
        self.temperature *= alpha;
    }
    // possibility to acceptance neighbor solution
    fn acceptance<O: PartialOrd + Into<f32> + Sub<Output = O>>(
        &mut self,
        fitness: &dyn Fn(&Solution<T>) -> O,
    ) {
        let neighbor = self.solution.neighbor();
        let delta: f32 = (fitness(&neighbor) - fitness(&self.solution)).into();
        if delta < 0.0 {
            self.solution = neighbor;
        } else {
            if fastrand::f32() < (-delta.abs() / self.temperature).exp() {
                self.solution = neighbor;
            }
        }
    }
}

impl<T, F, O> Annealer<T, F>
where
    F: Fn(&Solution<T>) -> O,
    O: PartialOrd + Into<f32> + Sub + Sub<Output = O>,
    T: Copy + Debug,
{
    fn init<R: Fn() -> Solution<T>>(config: AnnealerConfig, fitness: F, randness: R) -> Self {
        let state = AnnealState::initial_random_state(randness, config.temperature_zero);
        Annealer {
            config,
            state,
            fitness,
        }
    }
    fn anneal(&mut self) -> Solution<T> {
        for _ in 0..self.config.iteration {
            if self.state.temperature >= self.config.temperature_end {
                self.state.acceptance(&self.fitness);
            } else {
                break;
            }
        }
        self.state.solution.clone()
    }
}
