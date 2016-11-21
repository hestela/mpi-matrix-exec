use std::path::PathBuf;
use yaml_rust::{Yaml, yaml};

pub fn find_yaml_path(path: PathBuf) -> Option<PathBuf> {
  let has_build_yaml = path.join(".urbs.yaml").exists();
  let has_project_yaml = path.join(".urbsproj.yaml").exists();

  let yaml_path = match has_build_yaml {
    true if has_project_yaml == false => Some(path.join(".urbs.yaml")),
    false if has_project_yaml == true => Some(path.join(".urbsproj.yaml")),
    true if has_project_yaml == true => {
      println!("ERROR: There can not be both a .urbs.yaml \
               and .urbsproj.yaml in the same dir.");
      None
    },
    _ => {
      println!("ERROR: Could not find .urbs.yaml or .urbsproj.yaml in");
      println!("{}", path.display());
      None
    }
  };
  yaml_path
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
