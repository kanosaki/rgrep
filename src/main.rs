mod seeker;
mod feeder;

extern crate rustc_serialize;
extern crate docopt;
extern crate memmap;

use docopt::Docopt;
use feeder::Feeder;

const USAGE: &'static str = "
My grep by rust

Usage:
  rgrep <pattern> <files>...

Options:
  -R --recursive  Recursively grep directories.
  -e --regexp     Trait pattern as a regular expression.
  -g --glob       Trait pattern as unix glob. (default)
";

#[derive(Debug, RustcDecodable)]
struct Args {
  flag_recursive: bool,
  arg_pattern: String,
  arg_files: Vec<String>,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());
  println!("{:?}", args);
  let seeker = seeker::literal::Literal {
    lit: args.arg_pattern,
  };
  let feeder = Feeder::new(&args.arg_files[0]);
}
