//! This module implements a Lisp parser.

use crate::reader::Reader;
use std::str::Chars;

/// This enum is an ADT that defines is used to represent a Lisp token.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum Token {
  #[default]
  /// (E)nd (O)f (F)ile
  Eof,
  /// (Open) parenthesis or bracket
  Open,
  /// (Close) parenthesis or bracket
  Close,
  /// Single quote ('), used to interpret a Lisp list as data instead of code
  Quote,
  /// (Sym)bol, for variable names, function names and the like
  Sym(String),
  /// (Str)ing, a string literal
  Str(String),
  /// (Number), a number literal
  Number(f64),
}

/// The structure that stores the data used during the lexing phase
pub struct Lexer<'a> {
  // raw: String,
  chars: Reader<char, 2, Chars<'a>>,
  token: Option<Token>,
}

impl<'a> From<&'a str> for Lexer<'a> {
  /// Construct an uninitialised lexer that uses an `str` for input
  fn from(string: &'a str) -> Self {
    Self {
      // raw: string,
      chars: Reader::from(string.chars()),
      token: None,
    }
  }
}

impl Lexer<'_> {
  /// Initialise the lexer's state
  pub fn init(mut self) -> Self {
    self.token = self._next();
    self
  }
  fn _next(&mut self) -> Option<Token> {
    use Token::*;
    if let Some(c) = self.chars.peek(0) {
      let token = match c {
        '(' | '[' => Some(Open),
        ')' | ']' => Some(Close),
        '\'' => Some(Quote),
        '"' => return self.next_str(),
        '0'..='9' => return self.next_number(),
        ' ' | '\t' | '\n' => {
          self.chars.next();
          return self._next();
        }
        _ => return Some(Sym(self.next_sym())),
      };
      self.chars.next();
      token
    } else {
      None
    }
  }
  fn escape(&mut self, string: &mut String) -> bool {
    let ch = if let Some(c) = self.chars.next() {
      match c {
        'r' => '\r',
        'n' => '\n',
        '0' => '\0',
        'x' | 'X' => todo!("Hex escape sequences"),
        _ => return false,
      }
    } else {
      return false;
    };
    string.push(ch);
    true
  }
  fn next_str(&mut self) -> Option<Token> {
    let mut string = String::with_capacity(8);
    self.chars.next();
    loop {
      if let Some(c) = self.chars.next() {
        if c == '"' {
          break;
        } else if c == '\\' {
          if !self.escape(&mut string) {
            return None;
          }
        } else {
          string.push(c);
        }
      }
    }
    Some(Token::Str(string))
  }
  // TODO: Better number parsing (handle overflows)
  fn next_number(&mut self) -> Option<Token> {
    let mut value = 0f64;
    let mut exp = 1f64;
    loop {
      if let Some(c) = self.chars.peek(0) {
        if ('0'..='9').contains(&c) {
          value *= exp;
          value += <u32 as Into<f64>>::into(c as u32 - '0' as u32);
          exp *= 10f64;
          self.chars.next();
        } else {
          break;
        }
      } else {
        return Some(Token::Number(value));
      }
    }
    if Some('.') == self.chars.peek(0) {
      exp = 0.1f64;
      self.chars.next();
    }
    while let Some(c) = self.chars.peek(0) {
      if ('0'..='9').contains(&c) {
        value += <u32 as Into<f64>>::into(c as u32 - '0' as u32) * exp;
        exp /= 10f64;
        self.chars.next();
      } else {
        break;
      }
    }
    Some(Token::Number(value))
  }
  fn next_sym(&mut self) -> String {
    let mut sym = String::with_capacity(8);
    while let Some(c) = self.chars.peek(0) {
      if (c != ' '
        && c != '\t'
        && c != '\n'
        && c != '('
        && c != ')'
        && c != '['
        && c != ']')
      {
        sym.push(c);
        self.chars.next();
      } else {
        break;
      }
    }
    sym
  }
  pub fn peek(&self) -> &Option<Token> {
    &self.token
  }
}

impl Iterator for Lexer<'_> {
  type Item = Token;
  fn next(&mut self) -> Option<Self::Item> {
    if self.token.is_some() {
      let token = self.token.clone();
      self.token = self._next();
      token
    } else {
      None
    }
  }
}
