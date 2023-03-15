#[allow(unused_imports)]

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use lisptick::{Lexer, Token};

fn print_usage() {
  println!(
    r#"
    Usage: lisptick [file]
    file :: The file with the code
    "#
    );
}

fn main() -> io::Result<()> {
  match env::args().nth(1) {
    Some(path) => {
      let mut file = File::open(path)?;
      let mut string = String::with_capacity(16);
      file.read_to_string(&mut string)?;
      let lexer = Lexer::from(string.as_str()).init();
      for token in lexer.into_iter() {
        println!("{:#?}", token);
      }
    }
    None => print_usage()
  };
  Ok(())
}
