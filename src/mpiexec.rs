use std::path::PathBuf;
use std::fmt;
use yaml_rust::yaml;
use std::fs::File;
use std::io::prelude::*;
use mpi::Mpi;
use yamlutils;

pub struct MpiExec {
  name: String,
  work_dir: String,
  mpis: Vec<Mpi>,
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
    let mpirun_paths = yamlutils::get_yaml_vec(&yaml_doc["mpirun_paths"]).unwrap();
    let mpis = mpirun_paths.iter().map(|path| Mpi::new(path.clone())).collect();

    MpiExec {
      name: yamlutils::get_yaml_str(&yaml_doc["Name"]).unwrap(),
      work_dir: yamlutils::get_yaml_str(&yaml_doc["work_dir"]).unwrap(),
      mpis: mpis,
      env_vars: yamlutils::get_yaml_vec(&yaml_doc["envvars"]).unwrap(),
      run_opts: yamlutils::get_yaml_vec(&yaml_doc["runopts"]).unwrap(),
      np: yamlutils::get_yaml_vec(&yaml_doc["np"]).unwrap(),
    }
  }
}

impl fmt::Display for MpiExec {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {:?}, {:?}, {:?}, {:?})", self.name, self.work_dir,
      self.mpis, self.env_vars, self.run_opts, self.np)
  }
}
