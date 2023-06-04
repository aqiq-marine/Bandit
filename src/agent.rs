use rand::Rng;

pub struct Agent<const ARMS: usize> {
    epsilon: f64,
    alpha: f64,
    q_table: [f64; ARMS],
    play_freq: [f64; ARMS],
}

impl<const ARMS: usize> Agent<ARMS> {
    pub fn create(epsilon: f64, alpha: f64) -> Self {
        Self {
            epsilon,
            alpha,
            q_table: [0.0; ARMS],
            play_freq: [0.0; ARMS],
        }
    }
    pub fn update(&mut self, action: usize, reward: f64) {
        self.play_freq[action] += (1.0 - self.play_freq[action]) * self.alpha;
        self.q_table[action] += (reward - self.q_table[action]) * self.alpha;
    }

    pub fn pick_action(&self) -> usize {
        let rand_num: f64 = rand::thread_rng().gen();
        if rand_num < self.epsilon {
            rand::thread_rng().gen_range(0..ARMS)

            // 精度が悪い
            // self.play_freq.iter()
            //     .enumerate()
            //     .fold((0, 0.0), |(min_index, min_freq), (i, &freq)| {
            //         if freq < min_freq {
            //             (i, min_freq)
            //         } else {
            //             (min_index, min_freq)
            //         }
            //     }).0
        } else {
            let mut max = 0.0;
            let mut max_index = 0;
            for (i, q) in self.q_table.iter().enumerate() {
                if *q > max {
                    max = *q;
                    max_index = i;
                }
            }

            max_index
        }
    }
}


