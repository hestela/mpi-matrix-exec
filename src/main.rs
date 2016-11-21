extern crate argparse;
extern crate yaml_rust;

pub mod parse;
pub mod build;
pub mod project;
pub mod cmderror;
pub mod yamlutils;
pub mod debug;

use std::env;
use std::path::PathBuf;
use project::Project;

fn main() {
    let opts = parse::parse_opts();
    let check_path = match opts.path {
      Some(path) => PathBuf::from(path),
      None => PathBuf::from(env::current_dir().unwrap())
    };

    verbose!(println!("Checking for urbs yamls in: {}", check_path.display()), opts);

    let yaml_path_maybe = yamlutils::find_yaml_path(check_path);
    let yaml_path = match yaml_path_maybe {
      None => std::process::exit(1),
      _ => yaml_path_maybe.unwrap()
    };

    let proj = Project::new(&yaml_path);
}