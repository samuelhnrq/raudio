use std::fmt::Debug;
use std::str::FromStr;

pub fn extract_value<T>(args: &clap::ArgMatches, name: &str) -> T
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
