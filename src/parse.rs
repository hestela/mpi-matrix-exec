extern crate argparse;
use argparse::*;

pub struct Options {
  pub verbose: bool,
  pub path: Option<String>
}

macro_rules! get_ver {
  () => (
    "URBS Version: ".to_string() + &env!("CARGO_PKG_VERSION").to_string()
  )
}

pub fn parse_opts() -> Options {
  let mut opts = Options { verbose: false, path: None };

  {
  let mut parser = ArgumentParser::new();
  parser.set_description("URBS: Univeral Rust Build System");
  parser.refer(&mut opts.verbose)
    .add_option(&["-v", "--verbose"], StoreTrue,
                                  "Be verbose");
  parser.add_option(&["-V", "--version"],
    Print(get_ver!()), "Show version");

  parser.refer(&mut opts.path).add_argument("path", StoreOption,
                            "path to project or build dir");

  parser.parse_args_or_exit();
  }

  opts
}
