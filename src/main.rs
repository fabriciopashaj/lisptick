#[allow(unused_imports)]

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use lisptick::lexer::Lexer;
use lisptick::parser::Parser;

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
      let mut parser = Parser::from(Lexer::from(string.as_str()).init());
      let list = parser.parse_list();
      println!("{:#?}", list);
    }
    None => print_usage()
  };
  Ok(())
}
