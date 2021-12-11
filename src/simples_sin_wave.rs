use crate::args::extract_value;
use clap::ArgMatches;
use std::f64::consts::PI;

pub fn run(matches: &ArgMatches) {
  let n_samples: i32 = extract_value(&matches, "num_samples");
  let frequency: f64 = extract_value(&matches, "frequency");
  let sample_rate: f64 = extract_value(&matches, "sample_rate");

  let two_pi = 2.0 * PI;
  let angle = two_pi * frequency / sample_rate;
  for i in 0..n_samples {
    let sample = angle * i as f64;
    println!("{:.8}", sample.sin());
  }
  println!("done");
}
