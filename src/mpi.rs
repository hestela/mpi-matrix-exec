use std::fmt;

#[derive(Debug)]
enum MpiType {
  OpenMPI,
  // only supporing openmpi now
  Unk
}

#[derive(Debug)]
pub struct Mpi {
  mpi_type: MpiType,
  bin_path: String
}

impl Mpi {
  pub fn new(bin_path: String) -> Mpi {
    Mpi {
      mpi_type: MpiType::OpenMPI,
      bin_path: bin_path
    }
  }
}
