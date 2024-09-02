#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
  Up,
  Left,
  Down,
  Right,
}

impl Direction {
  pub fn move_to(&self, dir: &Self) -> Self {
    if self == dir || *self == dir.opposite() {
      self.clone()
    } else {
      dir.clone()
    }
  }

  pub const fn opposite(&self) -> Self {
    match self {
      Self::Up => Self::Down,
      Self::Left => Self::Right,
      Self::Down => Self::Up,
      Self::Right => Self::Left,
    }
  }
}
