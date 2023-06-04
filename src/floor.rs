use crate::bandit::*;
use crate::agent::*;

pub struct Floor<const ARMS: usize> {
    bandits: Bandit<ARMS>,
    bandit_p: [(f64, f64); ARMS],
    agent: Agent<ARMS>,
    agent_p: (f64, f64),
}

impl<const ARMS: usize> Floor<ARMS> {
    pub fn create(epsilon: f64, alpha: f64) -> Self {
        let bandits = Bandit::<ARMS>::new();
        let bandit_p = [(0.0, 0.0); ARMS];
        let agent = Agent::<ARMS>::create(epsilon, alpha);
        let agent_p = (0.0, 0.0);
        Self {
            bandits,
            bandit_p,
            agent,
            agent_p,
        }

    }
    pub fn step(&mut self) -> f64 {
        let action = self.agent.pick_action();
        // self.agent_p.0 += action as f64 - 1.0;
        // let p = self.agent_p.0.floor() as usize;
        // if p >= ARMS {
        //     return 0.0;
        // }
        let reward = self.bandits.play(action);
        self.agent.update(action, reward);
        
        reward

    }
}
