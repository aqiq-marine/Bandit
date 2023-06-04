use rand::{Rng, seq::SliceRandom};

pub struct Bandit<const ARMS: usize> {
    rates: [f64; ARMS],
}

impl<const ARMS: usize> Bandit<ARMS> {
    pub fn new() -> Self {
        let mut rnd = rand::thread_rng();
        let rates = [0.0; ARMS].map(|_| rnd.gen());
        let wtb = 0.5;
        let rates = Self::normalize_expected_value(rates, wtb);
        println!("{:?}", rates);
        Self { rates }
    }
    pub fn play(&mut self, arm: usize) -> f64 {
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
