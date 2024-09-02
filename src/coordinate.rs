use crate::{direction::Direction, BOARD_SIZE};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinate {
  pub x: usize,
  pub y: usize,
}

impl Coordinate {
  pub const fn new(x: isize, y: isize) -> Option<Self> {
    if x >= 0 && x < BOARD_SIZE as isize && y >= 0 && y < BOARD_SIZE as isize {
      Some(Self {
        x: x as usize,
        y: y as usize,
      })
    } else {
      None
    }
  }

  #[allow(clippy::cast_possible_wrap)]
  pub const fn r#move(&self, dir: &Direction) -> Option<Self> {
    match dir {
      Direction::Up => Self::new(self.x as isize, self.y as isize - 1),
      Direction::Left => Self::new(self.x as isize - 1, self.y as isize),
      Direction::Down => Self::new(self.x as isize, self.y as isize + 1),
      Direction::Right => Self::new(self.x as isize + 1, self.y as isize),
    }
  }
}
