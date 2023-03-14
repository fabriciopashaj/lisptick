pub mod reader;

#[allow(unused)]

use crate::reader::Reader;
use std::str::Chars;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Token {
  #[default]
  Eof,
  LParen,
  RParen,
  LBracket,
  RBracket,
  Quote,
  Sym(String),
  Str(String),
  Number(f64)
}

pub struct Lexer<'a> {
  // raw: String,
  chars: Reader<char, 2, Chars<'a>>,
  token: Option<Token>
}

impl<'a> From<&'a str> for Lexer<'a> {
  fn from(string: &'a str) -> Self {
    Self {
      // raw: string,
      chars: Reader::from(string.chars()),
      token: None
    }
  }
}

impl Lexer<'_> {
  pub fn init(mut self) -> Self {
    self.token = self._next();
    self
  }
  fn _next(&mut self) -> Option<Token> {
    use Token::*;
    if let Some(c) = self.chars.peek(0) {
      let token = match c {
        '('               => Some(LParen),
        ')'               => Some(RParen),
        '['               => Some(LBracket),
        ']'               => Some(RBracket),
        '\''              => Some(Quote),
        '"'               => return self.next_str(),
        '0'..='9'         => return self.next_number(),
        ' ' | '\t' | '\n' => { self.chars.next(); return self._next() },
        _                 => return Some(Sym(self.next_sym()))
      };
      self.chars.next();
      token
    } else { None }
  }
  fn escape(&mut self, string: &mut String) -> bool {
    let ch = if let Some(c) = self.chars.next() {
      match c {
        'r' => '\r',
        'n' => '\n',
        '0' => '\0',
        'x' | 'X' => todo!("Hex escape sequences"),
        _ => return false
      }
    } else { return false; };
    string.push(ch);
    return true;
  }
  fn next_str(&mut self) -> Option<Token> {
    let mut string = String::with_capacity(8);
    self.chars.next();
    loop {
      if let Some(c) = self.chars.next() {
        if c == '"' { break; }
        else if c == '\\' {
          if !self.escape(&mut string) { return None; }
        } else {
          string.push(c);
        }
      }
    }
    Some(Token::Str(string))
  }
  fn next_number(&mut self) -> Option<Token> {
    let mut value = 0f64;
    let mut exp = 1f64;
    loop {
      if let Some(c) = self.chars.peek(0) {
        if ('0'..='9').contains(&c) {
          value += <u32 as Into<f64>>::into(c as u32 - '0' as u32) * exp;
          exp *= 10f64;
          self.chars.next();
        } else { break; }
      } else { return Some(Token::Number(value)); }
    }
    if Some('.') == self.chars.peek(0) {
      exp = 0.1f64;
      self.chars.next();
    }
    loop {
      if let Some(c) = self.chars.peek(0) {
        if ('0'..='9').contains(&c) {
          value += <u32 as Into<f64>>::into(c as u32 - '0' as u32) * exp;
          exp /= 10f64;
          self.chars.next();
        } else { break; }
      } else { break; }
    }
    Some(Token::Number(value))
  }
  fn next_sym(&mut self) -> String {
    let mut sym = String::with_capacity(8);
    loop {
      if let Some(c) = self.chars.peek(0) {
        if c != ' ' && c != '\t' && c != '\n' && c != '(' && c != ')' {
          sym.push(c);
          self.chars.next();
        } else { break; }
      } else { break; }
    }
    sym
  }
}

impl Iterator for Lexer<'_> {
  type Item = Token;
  fn next(&mut self) -> Option<Self::Item> {
    if self.token.is_some() {
      let token = self.token.clone();
      self.token = self._next();
      token
    } else { None }
  }
}
