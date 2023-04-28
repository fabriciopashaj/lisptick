use std::fmt;
use std::collections::LinkedList;
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
  lexer: Lexer<'a>
}

#[derive(Debug, PartialEq)]
pub enum Node {
  Str(String),
  Sym(String),
  Number(f64),
  List(LinkedList<Node>)
}

impl<'a> From<Lexer<'a>> for Parser<'a> {
  fn from(lexer: Lexer<'a>) -> Self {
    Self { lexer }
  }
}

impl Parser<'_> {
  pub fn parse_node(&mut self) -> Node {
    if let Some(token) = self.lexer.next() {
      match token {
        Token::Eof => panic!("unlikely, start screaming"),
        Token::LParen => {
          self.parse_list()
        },
        Token::RParen => panic!("Unexpected ')' token"),
        Token::LBracket | Token::RBracket =>
          todo!("Don't know how to handle brackets"),
        Token::Quote =>
          Node::List([Node::Sym("quote".to_string()),
                      self.parse_node()].into()),
        Token::Sym(sym) => Node::Sym(sym),
        Token::Str(string) => Node::Str(string),
        Token::Number(num) => Node::Number(num)
      }
    } else {
      Node::List(LinkedList::new())
    }
  }
  pub fn parse_list(&mut self) -> Node {
    let mut list = LinkedList::new();
    while let Some(token) = self.lexer.peek() {
      if let Token::RParen = token {
        self.lexer.next();
        break;
      } else {
        list.push_back(self.parse_node());
      }
    }
    Node::List(list)
  }
}

#[cfg(test)]
mod tests {
  use super::{super::lexer::Lexer, Parser, Node};
  use std::collections::LinkedList;
  #[test]
  fn test_parser() {
    let mut parser = Parser::from(Lexer::from("(foo bar (baz 4))").init());
    let list = parser.parse_node();
    assert_eq!(list,
               Node::List([
                 Node::Sym("foo".to_string()),
                 Node::Sym("bar".to_string()),
                 Node::List([
                    Node::Sym("baz".to_string()),
                    Node::Number(4f64)
                 ].into())
               ].into()));
    let mut parser = Parser::from(
                       Lexer::from("(car '(a b 23.342 \"foo\"))").init());
    let list = parser.parse_node();
    assert_eq!(list,
               Node::List([
                 Node::Sym("car".to_string()),
                 Node::List([
                   Node::Sym("quote".to_string()),
                   Node::List([
                     Node::Sym("a".to_string()),
                     Node::Sym("b".to_string()),
                     Node::Number(23.342f64),
                     Node::Str("foo".to_string())
                   ].into())
                 ].into())
               ].into()));
  }
}
