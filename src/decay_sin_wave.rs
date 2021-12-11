use crate::args::extract_value;
use clap::ArgMatches;
use std::f32::consts::PI;
use std::time::Instant;

const TWO_PI: f32 = PI * 2.0;
const AMPLITUDE: f32 = std::i16::MAX as f32;

pub fn run(matches: &ArgMatches) {
  let duration: f32 = extract_value(&matches, "duration");
  let frequency: f32 = extract_value(&matches, "frequency");
  let sample_rate: f32 = extract_value(&matches, "sample_rate");
  let slope: f32 = extract_value(&matches, "slope");

  let spec = hound::WavSpec {
    channels: 1,
    sample_rate: sample_rate.round() as u32,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };

  let total_samples = (duration * sample_rate).round() as usize;
  let angle_inc = TWO_PI * frequency / sample_rate;

  let sample_rate = duration / total_samples as f32;
  let decay = (-sample_rate / slope).exp();

  let mut writer =
    hound::WavWriter::create("sine.wav", spec).expect("Could not create file for writing");

  let mut x = 1.0;
  let start_time = Instant::now();
  println!("File opened pre-req calculated will start processing now.");

  for i in 0..total_samples {
    x *= decay;
    let sample = (angle_inc * i as f32).sin() * x;
    writer
      .write_sample((sample * AMPLITUDE) as i16)
      .expect("Failed to write to file");
  }
  println!(
    "Done generating sound, took {} us",
    start_time.elapsed().as_micros()
  );
}
