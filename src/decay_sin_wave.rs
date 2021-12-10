use clap::ArgMatches;

use crate::args::value_as;

use std::f32::consts::PI;
const TWO_PI: f32 = PI * 2.0;

pub fn run(matches: &ArgMatches) {
  let duration: f32 = value_as(&matches, "duration");
  let frequency: f32 = value_as(&matches, "frequency");
  let sample_rate: f32 = value_as(&matches, "sample_rate");
  let slope: f32 = value_as(&matches, "slope");

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: sample_rate.round() as u32,
    bits_per_sample: 32,
    sample_format: hound::SampleFormat::Float,
  };

  let n_samples = (duration * sample_rate).round() as usize;
  let angle_inc = TWO_PI * frequency / sample_rate;

  let bit_rate = duration / n_samples as f32;
  let decay = (-bit_rate / slope).exp();

  let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

  let mut x = 1.0;

  for i in 0..n_samples {
    x *= decay;
    let sample = (angle_inc * i as f32).sin() * x;
    writer
      .write_sample(sample)
      .expect("Failed to write to file");
  }
  println!("done");
}
