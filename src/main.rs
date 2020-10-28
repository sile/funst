use anyhow::Context;
use ordered_float::OrderedFloat;
use std::io::BufRead;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Calculate fundamental statistics of numbers given via the standard input.")]
struct Opt {}

fn main() -> anyhow::Result<()> {
    let _opt = Opt::from_args();
    let mut values = Vec::new();

    for (i, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.with_context(|| format!("Failed to read the line {}", i + 1))?;
        let value: f64 = line
            .parse()
            .with_context(|| format!("Line {} is not a number: {:?}", i + 1, line))?;
        anyhow::ensure!(
            value.is_finite(),
            "Line {} is not a finite number: {}",
            i + 1,
            value
        );
        values.push(value);
    }
    anyhow::ensure!(!values.is_empty(), "Empty standard input");

    values.sort_by_key(|v| OrderedFloat(*v));
    let n = values.len();
    let stat = Stat {
        count: n,
        mean: mean(&values),
        stddev: stddev(&values),
        min: values[0],
        median: if n % 2 == 1 {
            values[n / 2]
        } else {
            (values[n / 2 - 1] + values[n / 2]) / 2.0
        },
        max: values[n - 1],
    };
    serde_json::to_writer_pretty(std::io::stdout().lock(), &stat)?;
    println!();

    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct Stat {
    count: usize,
    mean: f64,
    stddev: f64,
    min: f64,
    median: f64,
    max: f64,
}

fn mean(vs: &[f64]) -> f64 {
    vs.iter().sum::<f64>() / vs.len() as f64
}

fn stddev(vs: &[f64]) -> f64 {
    let n = vs.len() as f64;
    let m = mean(vs);
    (vs.iter().map(|&v| (v - m).powi(2)).sum::<f64>() / n).sqrt()
}
