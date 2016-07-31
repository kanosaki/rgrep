
use memmap::{Mmap, Protection};
use std::io;

pub struct Feeder {
  mmap: Mmap,
}

impl Feeder {
  pub fn new(path: &String) -> io::Result<Feeder> {

    let mmap = match Mmap::open_path(path, Protection::Read) {
      Err(e) => return Err(e),
      Ok(f_map) => f_map
    };
    return Ok(Feeder{
      mmap: mmap,
    })
  }
}

