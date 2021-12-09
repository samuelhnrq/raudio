#[macro_use]
extern crate clap;

mod args;
mod simples_sin_wave;

use clap::App;

fn main() {
  let yaml = load_yaml!("../cli.yaml");
  let matches = App::from_yaml(yaml).get_matches();
  match matches.subcommand() {
    ("simple_sin", Some(matches)) => {
      simples_sin_wave::run(matches);
    }
    _ => {
      println!("{}", matches.usage());
    }
  }
}
