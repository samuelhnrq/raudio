#[macro_use]
extern crate clap;

mod hound_f32;

mod args;
mod change_amplitude;
mod decay_sin_wave;
mod simples_sin_wave;

use clap::App;

fn main() {
  let yaml = load_yaml!("../cli.yaml");
  let matches = App::from_yaml(yaml).get_matches();
  match matches.subcommand() {
    ("simple_sin", Some(matches)) => {
      simples_sin_wave::run(matches);
    }
    ("decay_sin", Some(matches)) => {
      decay_sin_wave::run(matches);
    }
    ("change_amplitude", Some(matches)) => {
      change_amplitude::run(matches);
    }
    _ => {
      println!("{}", matches.usage());
    }
  }
}
