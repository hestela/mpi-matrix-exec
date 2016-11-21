use std::path::PathBuf;
use std::fmt;
use yaml_rust::yaml;
use std::fs::File;
use std::io::prelude::*;
use yamlutils;

pub struct MpiExec {
  name: String,
  mpi: String,
  env_vars: Vec<String>,
  run_opts: Vec<String>,
  np: Vec<String>
}

impl MpiExec {
  pub fn new(path: &PathBuf) -> MpiExec {
    let mut file = File::open(path).unwrap();
    let mut out = String::new();
    let _ = file.read_to_string(&mut out);
    let yaml_data = yaml::YamlLoader::load_from_str(&out).unwrap();

    // Only using one document per yaml file at the moment.
    let yaml_doc = &yaml_data[0];

    MpiExec {
      name: yamlutils::get_yaml_str(&yaml_doc["Name"]).unwrap(),
      mpi: yamlutils::get_yaml_str(&yaml_doc["MPI"]).unwrap(),
      env_vars: yamlutils::get_yaml_vec(&yaml_doc["envvars"]).unwrap(),
      run_opts: yamlutils::get_yaml_vec(&yaml_doc["runopts"]).unwrap(),
      np: yamlutils::get_yaml_vec(&yaml_doc["np"]).unwrap(),
    }
  }
}

impl fmt::Display for MpiExec {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {:?}, {:?}, {:?})", self.name, self.mpi, self.env_vars,
      self.run_opts, self.np)
  }
}
