use plotters::prelude::*;

mod bandit;
mod agent;
mod floor;
use crate::floor::*;

const ARMS: usize = 10;

fn runner() -> Result<(), Box<dyn std::error::Error>> {
    let steps = 100_000;
    let epsilon = 0.10;
    let alpha = 0.5;

    let mut floor = Floor::<ARMS>::create(epsilon, alpha);

    let ema_n = steps as usize / 1000;
    let mut emas = vec![1.0 / ARMS as f64];
    let a = 2.0 / (ema_n as f64 + 1.0);

    for _ in 0..steps {
        let reward = floor.step();

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
