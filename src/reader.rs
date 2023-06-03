pub struct Reader<T, const L: usize, I>
where
  T: Default + std::marker::Copy,
  I: Iterator<Item = T>,
{
  i: usize,
  high: usize,
  buf: [T; L],
  curr: Option<T>,
  src: I,
}

impl<T, const L: usize, I> From<I> for Reader<T, L, I>
where
  T: Default + std::marker::Copy,
  I: Iterator<Item = T>,
{
  fn from(mut src: I) -> Self {
    let mut buf = [T::default(); L];
    let mut high = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..L {
      if let Some(v) = src.next() {
        buf[i] = v;
        high = i;
      } else {
        break;
      }
    }
    Self {
      buf,
      high,
      src,
      i: 0,
      curr: None,
    }
  }
}

impl<T, const L: usize, I> Iterator for Reader<T, L, I>
where
  T: Default + std::marker::Copy,
  I: Iterator<Item = T>,
{
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    let item = self._next();
    self.curr = item;
    item
  }
}

impl<T, const L: usize, I> Reader<T, L, I>
where
  T: Default + std::marker::Copy,
  I: Iterator<Item = T>,
{
  pub fn peek(&self, n: usize) -> Option<T> {
    if self.i == self.high || n > L - 1 {
      None
    } else {
      Some(self.buf[(self.i + n) % L])
    }
  }
  pub fn curr(&self) -> Option<T> {
    self.curr
  }
  fn _next(&mut self) -> Option<T> {
    if self.high == self.i {
      None
    } else if let Some(item) = self.src.next() {
      let r = self.buf[self.i];
      self.buf[self.i] = item;
      self.i = (self.i + 1) % L;
      self.high = (self.high + 1) % L;
      Some(r)
    } else {
      let r = self.buf[self.i];
      self.i = (self.i + 1) % L;
      Some(r)
    }
  }
}
