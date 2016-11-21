#![macro_use]
macro_rules! verbose {
  ($expr:expr,$opts:ident) => (
    match $opts.verbose {
      true => $expr,
      false => ()
    }
  )
}
