#![allow(dead_code)]
use crate::SaResult;
/// Generic Algorithm in Rust
use std::fmt::Debug;

pub trait Solution: Clone + Debug {
    fn fitness(&self) -> f32;
    // rand solution
    fn rand() -> Self;
    // random neighbor solution
    fn neighbor(&self) -> Self;
}

pub struct AnnealState<S>
where
    S: Solution,
{
    solution: S,
    temperature: f32,
}

impl<S> AnnealState<S>
where
    S: Solution,
{
    fn initial(config: &AnnealerConfig) -> Self {
        let temperature = config.temperature_zero;
        let solution = S::rand();
        AnnealState {
            solution,
            temperature,
        }
    }

    fn update_temperature(&mut self, config: &AnnealerConfig) {
        self.temperature *= config.alpha;
    }

    // possibility to acceptance neighbor solution
    fn acceptance(&mut self) {
        let neighbor = self.solution.neighbor();
        let delta = neighbor.fitness() - self.solution.fitness();
        if delta < 0.0 {
            self.solution = neighbor;
        } else {
            if rand::random::<f32>() < (-delta.abs() / self.temperature).exp() {
                self.solution = neighbor;
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnealerConfig {
    alpha: f32,
    temperature_zero: f32,
    temperature_end: f32,
    iteration: usize,
}

pub struct Annealer<S: Solution> {
    config: AnnealerConfig,
    state: AnnealState<S>,
}

use std::{fs, path::Path};
impl<S: Solution> Annealer<S> {
    fn init<P: AsRef<Path>>(config_path: P) -> SaResult<Self> {
        let cfg_str: String = fs::read_to_string(config_path)?;
        let config: AnnealerConfig = serde_yaml::from_str(&cfg_str)?;
        let state = AnnealState::<S>::initial(&config);
        Ok(Annealer { config, state })
    }

    fn anneal(&mut self) -> SaResult<S> {
        for _ in 0..self.config.iteration {
            if self.state.temperature >= self.config.temperature_end {
                self.state.acceptance();
            } else {
                break;
            }
        }
        Ok(self.state.solution.clone())
    }
}

pub type SaResult<T> = std::result::Result<T, error::SaError>;
