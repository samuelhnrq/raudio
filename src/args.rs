use std::fmt::Debug;
use std::str::FromStr;

pub fn value_as<T>(args: &clap::ArgMatches, name: &str) -> T
where
  T: FromStr,
  <T as FromStr>::Err: Debug,
{
  args
    .value_of(name)
    .map(|x| x.parse::<T>().ok())
    .flatten()
    .unwrap()
}
