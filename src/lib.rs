use std::collections::HashMap;
use pyo3::prelude::*;
use rand::{
    self,
    distributions::{Distribution, Uniform},
};

#[pymodule]
fn pattingzoo_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Env>()?;
    Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[pyclass]
pub enum AgentType {
    Prisoner,
    Guard,
}

#[pyclass]
pub struct Observations {
    observation: (usize, usize, usize),
    action_mask: (usize, usize, usize, usize),
}

#[pymethods]
impl Observations {

    #[new]
    pub fn new(agent_type: AgentType, x: usize, y: usize) -> Observations {
        let observation = (0, 56, x + 7 * y);
        match agent_type {
            AgentType::Prisoner => Observations { observation: observation, action_mask: (0,1,1,0) },
            AgentType::Guard => Observations { observation: observation, action_mask: (1,0,0,1) },
        }
        
    }
}

#[derive(Default)]
#[pyclass(subclass)]
pub struct Env {
    #[pyo3(get)]
    escape_y: usize,
    #[pyo3(get)]
    escape_x: usize,
    #[pyo3(get)]
    guard_y: usize,
    #[pyo3(get)]
    guard_x: usize,
    #[pyo3(get)]
    prisoner_y: usize,
    #[pyo3(get)]
    prisoner_x: usize,
    timestep: usize,
    #[pyo3(get, set)]
    max_cycles: usize,
    #[pyo3(get, set)]
    possible_agents: Vec<String>,
    #[pyo3(get, set)]
    agents: Vec<String>,
}


#[pymethods]
impl Env {
    #[new]
    pub fn new() -> Env {
        Env {
            possible_agents: vec!["Prisoner".to_string(), "Guard".to_string()],
            agents: vec!["Prisoner".to_string(), "Guard".to_string()],
            ..Default::default()
        }
    }

    pub fn reset(&mut self, seed: Option<PyObject>, options: Option<PyObject>) -> (
        HashMap<String, Observations>,
        HashMap<String, String>
    ) {
        self.timestep = 0;
        self.prisoner_x = 0;
        self.prisoner_y = 0;
        self.guard_x = 0;
        self.guard_y = 0;
        
        self.agents = self.possible_agents.clone();

        let mut rng = rand::thread_rng();
        let sampler = Uniform::new_inclusive(2, 4);
        self.escape_x = sampler.sample(&mut rng);
        self.escape_y = sampler.sample(&mut rng);

        let observations = HashMap::from([
            ("Prisoner".to_string(), Observations::new(AgentType::Prisoner, self.escape_x, self.escape_y)),
            ("Guard".to_string(), Observations::new(AgentType::Guard, self.escape_x, self.escape_y)),
        ]);
        let infos = HashMap::from([
            ("Prisoner".to_string(), "".to_string()),
            ("Guard".to_string(), "".to_string()),
        ]);
        (observations, infos)
    }

    fn generate_prisoner_mask(&self) -> (usize, usize, usize, usize) {
        let left: usize = match self.prisoner_x {
            0 => 0,
            _ => 1,
        };
        let right: usize = match self.prisoner_x {
            6 => 0,
            _ => 1,
        };
        let down: usize = match self.prisoner_y {
            0 => 0,
            _ => 1,
        };
        let up: usize = match self.prisoner_y {
            6 => 0,
            _ => 1,
        };
        (left, right, down, up)
    }
    
    fn generate_guard_mask(&self) -> (usize, usize, usize, usize) {
        let left: usize = if (self.guard_y == self.escape_y) & (self.guard_x == self.escape_x + 1) {
            0
        } else {
            1
        };
        let right: usize = if (self.guard_y == self.escape_y) & (self.guard_x + 1 == self.escape_x) {
            0
        } else {
            1
        };
        let down: usize = if (self.guard_x == self.escape_x) & (self.guard_y == self.escape_y + 1) {
            0
        } else {
            1
        };
        let up: usize = if (self.guard_x == self.escape_x) & (self.guard_y + 1 == self.escape_y) {
            0
        } else {
            1
        };
        (left, right, down, up)
    }

    fn prisoner_escaped(&self) -> bool {
        if (self.prisoner_x == self.escape_x) & (self.prisoner_y == self.escape_y) {
            true
        } else {
            false
        }
    }

    fn prisoner_captured(&self) -> bool {
        if (self.prisoner_x == self.guard_x) & (self.prisoner_y == self.guard_y) {
            true
        } else {
            false
        }
    }

    pub fn step(&mut self, actions: HashMap<&str, usize>) -> (HashMap<String, Observations>, HashMap<String, isize>, HashMap<String, bool>, HashMap<String, bool>, HashMap<String, String>) {
        let prisoner_action = actions.get("Prisoner").unwrap();
        let guard_action = actions.get("Guard").unwrap();

        match prisoner_action {
            &0 if self.prisoner_x > 0 => self.prisoner_x -= 1,
            &1 if self.prisoner_x < 6 => self.prisoner_x += 1,
            &2 if self.prisoner_y > 0 => self.prisoner_y -= 1,
            &3 if self.prisoner_y < 6 => self.prisoner_y += 1,
            _ => (),
        };
        match guard_action {
            &0 if self.guard_x > 0 => self.guard_x -= 1,
            &1 if self.guard_x < 6 => self.guard_x += 1,
            &2 if self.guard_y > 0 => self.guard_y -= 1,
            &3 if self.guard_y < 6 => self.guard_y += 1,
            _ => (),
        };

        let prisoner_action_mask = self.generate_prisoner_mask();
        let guard_action_mask = self.generate_guard_mask();
        let observation = (
            self.prisoner_x + 7 * self.prisoner_y,
            self.guard_x + 7 * self.guard_y,
            self.escape_x + 7 * self.escape_y,
        );
        let observations = HashMap::from([
            ("Prisoner".to_string(), Observations {observation, action_mask: prisoner_action_mask }),
            ("Guard".to_string(), Observations {observation, action_mask: guard_action_mask }),
        ]);
        
        let (terminations, truncations, rewards) = if self.prisoner_escaped() {
            self.agents = vec![];
            (
                HashMap::from([("Prisoner".to_string(),true), ("Guard".to_string(), true)]),
                HashMap::from([("Prisoner".to_string(),false), ("Guard".to_string(), false)]),
                HashMap::from([("Prisoner".to_string(), 1), ("Guard".to_string(), -1)]),
            )
            
        } else if self.prisoner_captured() {
            self.agents = vec![];
            (
                HashMap::from([("Prisoner".to_string(), true), ("Guard".to_string(), true)]),
                HashMap::from([("Prisoner".to_string(),false), ("Guard".to_string(), false)]),
                HashMap::from([("Prisoner".to_string(), -1), ("Guard".to_string(), 1)]),
            )
        } else if self.timestep > 100 {
            self.agents = vec![];
            (
                HashMap::from([("Prisoner".to_string(),false), ("Guard".to_string(), false)]),
                HashMap::from([("Prisoner".to_string(), true), ("Guard".to_string(), true)]),
                HashMap::from([("Prisoner".to_string(), 0), ("Guard".to_string(), 0)]),
            )
        } else {
            (
                HashMap::from([("Prisoner".to_string(),false), ("Guard".to_string(), false)]),
                HashMap::from([("Prisoner".to_string(),false), ("Guard".to_string(), false)]),
                HashMap::from([("Prisoner".to_string(), 0), ("Guard".to_string(), 0)]),
            )
        };
        self.timestep += 1;

        let infos = HashMap::from([
            ("Prisoner".to_string(), "".to_string()),
            ("Guard".to_string(), "".to_string()),
        ]);
        
        (observations, rewards, terminations, truncations, infos)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sim_steps() {
        let mut env = Env::new();
        let (obs, infos) = env.reset(None, None);
        for _ in 0..99 {
            let actions = HashMap::from([
                ("Prisoner", 0),
                ("Guard", 0),
            ]);
            let (obs, rewards, terms, truncs, infos) = env.step(actions);
        };
    }
}

