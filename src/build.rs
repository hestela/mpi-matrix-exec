use std::path::PathBuf;
use std::process::Command;
use yaml_rust::{yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use yamlutils;

pub struct Build {
  name: String,
  path: PathBuf,
  build_cmds: Vec<String>,
  deploy_cmds: Vec<String>,
  test_cmds: Vec<String>
}

impl Build {
  pub fn new(path: &PathBuf) -> Build {
    let mut file = File::open(path).unwrap();
    let mut out = String::new();
    let _ = file.read_to_string(&mut out);
    let yaml_data = yaml::YamlLoader::load_from_str(&out).unwrap();

    // Only using one document per yaml file at the moment.
    let yaml_doc = &yaml_data[0];

    Build { name: String::new(), path: PathBuf::from(path),
      build_cmds: yamlutils::yaml_to_cmds(&yaml_doc["build_cmds"]),
      deploy_cmds: yamlutils::yaml_to_cmds(&yaml_doc["deploy_cmds"]),
      test_cmds: yamlutils::yaml_to_cmds(&yaml_doc["test_cmds"]),
    }
  }
}
