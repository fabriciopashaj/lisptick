#[allow(unused_imports)]

use lisptick::{Lexer, Token};

fn main() {
  let lexer = Lexer::from("((foo (+ 1.44) 'a) \"bar\")").init();
  for token in lexer.into_iter() {
    println!("{:#?}", token);
  }
}
