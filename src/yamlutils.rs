use std::path::PathBuf;
use yaml_rust::{Yaml, yaml};

pub fn find_yaml_path(path: PathBuf) -> Option<PathBuf> {
  let yaml = path.join("mpiexec.yaml");
  match yaml.exists() {
    true => Some(yaml),
    _ => {
      println!("ERROR: Could not find mpiexec.yaml in");
      println!("{}", path.display());
      None
    }
  }
}

pub fn yaml_to_cmds(yaml: &Yaml) -> Vec<String> {
  match yaml {
    &yaml::Yaml::Array(ref v) => {
      v.into_iter().map(|x| x.as_str().unwrap_or("")).filter(|x| x.len() > 0);
    },
    &yaml::Yaml::Hash(_) => {
      println!("ERROR: Commands should be a yaml array, not a hash");
    },
    _ => {
      println!("ERROR: Unkown input type for command");
    }
  }
  Vec::<String>::new()
}
