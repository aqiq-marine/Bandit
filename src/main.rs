use plotters::prelude::*;
use rand::{Rng, seq::SliceRandom};

struct Bandit<const ARMS: usize> {
    rates: [f64; ARMS],
}

impl<const ARMS: usize> Bandit<ARMS> {
    fn new() -> Self {
        let mut rnd = rand::thread_rng();
        let rates = [0.0; ARMS].map(|_| rnd.gen());
        let wtb = 0.5;
        let rates = Self::normalize_expected_value(rates, wtb);
        println!("{:?}", rates);
        Self { rates }
    }
    fn play(&mut self, arm: usize) -> f64 {
        let a = 0.01;
        self.rates
            .iter_mut()
            .for_each(|p| *p = (1.0 - a) * *p + a * rand::thread_rng().gen::<f64>());
        let wtb = 0.5;
        self.rates = Self::normalize_expected_value(self.rates, wtb);

        let rate: f64 = self.rates[arm.min(ARMS - 1)];
        let rand_num: f64 = rand::thread_rng().gen();
        if rand_num < rate {
            1.0
        } else {
            0.0
        }
    }
    fn normalize_expected_value(mut rates: [f64; ARMS], wtb: f64) -> [f64; ARMS] {

        // 分散させる -------------------
        let mut rng = rand::thread_rng();
        let mut x = vec![0; ARMS];
        x.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        x.shuffle(&mut rng);

        // vi = d xi / dt
        // v0 + v1 + v2 = 0
        // x0v0 + x1v1 + x2v2 > 0

        let mut v = [0.0; 3];
        v[0] = 1.0;

        if rates[x[2]] - rates[x[1]] != 0.0 {
            v[2] = (rates[x[1]] - rates[x[0]]) / (rates[x[2]] - rates[x[1]]) + 0.01;
            v[1] = -v[0] - v[2];

            let v_max = v.iter().fold(0.0, |max, vi| if vi.abs() > max {vi.abs()} else {max});
            let v_wtb = 0.1;
            let v = v.map(|vi| vi * v_wtb / v_max);

            rates[x[0]] += v[0];
            rates[x[1]] += v[1];
            rates[x[2]] += v[2];
        }
       
        // 0<xにする --------------------

        let min = rates.iter().fold(1.0, |min, &x| if x < min {x} else {min});
        
        if min < 0.0 {
            rates.iter_mut().for_each(|r| *r = *r - min);
        }

        // 期待値をwtb(want to be)にする ----------

        let expected_value = rates.iter().sum::<f64>() / ARMS as f64;

        rates.map(|r| r * wtb / expected_value)
    }
}

struct Agent<const ARMS: usize> {
    epsilon: f64,
    alpha: f64,
    q_table: [f64; ARMS],
    play_counts: [u32; ARMS],
}

impl<const ARMS: usize> Agent<ARMS> {
    fn create(epsilon: f64) -> Self {
        Self {
            epsilon,
            alpha: 0.5,
            q_table: [0.0; ARMS],
            play_counts: [0; ARMS],
        }
    }
    fn update(&mut self, action: usize, reward: f64) {
        self.play_counts[action] += 1;
        self.q_table[action] += (reward - self.q_table[action]) * self.alpha;
    }

    fn pick_action(&self) -> usize {
        let rand_num: f64 = rand::thread_rng().gen();
        if rand_num < self.epsilon {
            rand::thread_rng().gen_range(0..ARMS)
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

const ARMS: usize = 10;
const EMA_N: usize = 50;

fn runner() -> Result<(), Box<dyn std::error::Error>> {
    let steps = 100_000;
    let epsilon = 0.10;

    let mut bandit = Bandit::<ARMS>::new();
    let mut agent = Agent::<ARMS>::create(epsilon);

    let mut emas = vec![1.0 / ARMS as f64];
    let a = 2.0 / (EMA_N as f64 + 1.0);

    for step in 0..steps {
        let action = agent.pick_action();
        let reward = bandit.play(action);
        agent.update(action, reward);

        let ema = a * reward + (1.0 - a) * emas.last().unwrap_or(&0.0);
        emas.push(ema);
    }

    let points = emas
        .into_iter()
        .enumerate()
        .map(|(i, r)| (i as f64, r))
        .collect::<Vec<_>>();

    let root = BitMapBackend::new("ema.png", (512, 512)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Non Stat Bandit EMA", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..steps as f64, 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(points, &BLUE))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    runner()
}
