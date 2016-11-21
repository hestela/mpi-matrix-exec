use std::path::PathBuf;
use std::fmt::Display;
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

pub fn get_yaml_vec(yaml: &Yaml) -> Option<Vec<String>> {
  match yaml {
    &yaml::Yaml::Array(ref v) => {
      Some(v.into_iter().map(|x| x.clone().into_string().unwrap_or(String::new())).filter(|x| x.len() > 0).collect())
    },
    &yaml::Yaml::Hash(_) => {
      println!("ERROR: Commands should be a yaml array, not a hash");
      None
    },
    _ => {
      println!("ERROR: Wrong input type");
      None
    }
  }
}

pub fn get_yaml_str(yaml: &Yaml) -> Option<String> {
  match yaml {
    &yaml::Yaml::String(ref v) => {
      Some(v.clone())
    },
    _ => {
      println!("ERROR: Wrong input type");
      None
    }
  }
}
