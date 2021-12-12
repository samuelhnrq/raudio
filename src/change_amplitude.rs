use crate::args::extract_value;
use crate::hound_f32::{ensure_f32_samples, F32HoundWriter};
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
  let input_file: String = extract_value(&matches, "input_file");
  let change_factor: f32 = extract_value(&matches, "change_factor");

  if change_factor < 0.0 || change_factor > 1.0 {
    println!("Change factor must be between 0.0 and 1.0");
    return;
  }

  let reader = hound::WavReader::open(input_file).expect("Failed to open source file");
  let spec = reader.spec();
  let mut dest =
    F32HoundWriter::create("changed_vol.wav", spec).expect("Failed to create destination file");

  for sample in ensure_f32_samples(reader) {
    dest
      .write_sample(sample * change_factor)
      .expect("failed to write sample");
  }
}
