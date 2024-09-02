use {
  crate::{coordinate::Coordinate, direction::Direction, progress::Progress, BOARD_SIZE},
  rand::{rngs::ThreadRng, Rng},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snake {
  pub cells: Vec<Coordinate>,
  direction: Direction,
  pub fruit: Coordinate,
}

impl Snake {
  pub fn new() -> Self {
    #[expect(clippy::unwrap_used)]
    let mut snake: Self = Self {
      cells: vec![],
      direction: Direction::Right,
      fruit: Coordinate::new(0, 0).unwrap(),
    };
    snake.spawn_snake();
    snake.respawn_fruit();
    snake
  }

  pub fn change_direction(&mut self, dir: &Direction) {
    self.direction = self.direction.move_to(dir);
  }

  #[expect(clippy::expect_used)]
  fn head(&self) -> &Coordinate {
    self.cells.first().expect("No head")
  }

  pub fn r#move(&mut self) -> Progress {
    if let Some(head) = self.head().r#move(&self.direction) {
      let last = self.cells.len() - 1;
      for cell in &self.cells[0..last] {
        if head == *cell {
          return Progress::Lost;
        }
      }
      self.cells.insert(0, head);
      if self.is_eating() {
        self.respawn_fruit();
      } else {
        self.cells.pop();
      }
      Progress::Ongoing
    } else {
      Progress::Lost
    }
  }

  pub fn is_eating(&self) -> bool {
    self.cells[0] == self.fruit
  }

  fn respawn_fruit(&mut self) {
    let mut rng: ThreadRng = rand::thread_rng();
    while self.fruit_within_snake() {
      let x: isize = rng.gen_range(0..BOARD_SIZE as isize);
      let y: isize = rng.gen_range(0..BOARD_SIZE as isize);

      self.fruit = Coordinate::new(x, y).unwrap();
    }
  }

  fn spawn_snake(&mut self) {
    let mut rng: ThreadRng = rand::thread_rng();
    let x = rng.gen_range(2..BOARD_SIZE as isize - 2);
    let y = rng.gen_range(2..BOARD_SIZE as isize - 2);
    let dir = match rng.gen_range(0..4) {
      0 => Direction::Up,
      1 => Direction::Left,
      2 => Direction::Down,
      3 => Direction::Right,
      _ => unreachable!(),
    };
    let cells = vec![Coordinate::new(x, y).unwrap()];

    self.direction = dir.opposite();
    self.cells = cells;
  }

  fn fruit_within_snake(&self) -> bool {
    for cell in &self.cells {
      if *cell == self.fruit {
        return true;
      }
    }
    false
  }
}
