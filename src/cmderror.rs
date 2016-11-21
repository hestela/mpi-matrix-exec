use mpiexec::MpiExec;

pub struct CmdError {
  mpiexec: MpiExec,
  err_str: String
}
