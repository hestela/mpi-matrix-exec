extern crate argparse;
extern crate yaml_rust;

pub mod parse;
pub mod mpiexec;
pub mod cmderror;
pub mod yamlutils;
pub mod debug;

use std::env;
use std::path::PathBuf;

fn main() {
    let opts = parse::parse_opts();
    let check_path = match opts.path {
      Some(path) => PathBuf::from(path),
      None => PathBuf::from(env::current_dir().unwrap())
    };

    verbose!(println!("Checking for mpiexec yaml in: {}", check_path.display()), opts);

    let yaml_path_maybe = yamlutils::find_yaml_path(check_path);
    let yaml_path = match yaml_path_maybe {
      None => std::process::exit(1),
      _ => yaml_path_maybe.unwrap()
    };

    let mpiexec = mpiexec::MpiExec::new(&yaml_path);
    verbose!(println!("YAML options: {}", mpiexec), opts);
}
