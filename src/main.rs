#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::expect_used)]
#![deny(clippy::min_ident_chars)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]

mod coordinate;
mod direction;
mod front;
mod progress;
mod snake;

use {front::Front, snake::Snake};

const BOARD_SIZE: u8 = 32;
const CELL_SIZE: u8 = 16;
const CELL_SIZE_I32: i32 = 16;
const HEAD_SIZE_U32: u32 = 14;
const BODY_SIZE_U32: u32 = 12;
const WINDOW_SIZE: u32 = BOARD_SIZE as u32 * CELL_SIZE as u32;

fn main() -> Result<(), String> {
  let mut front: Front = Front::new()?;
  let mut game: Snake = Snake::new();
  front.initial_present();
  front.r#loop(&mut game);
  Ok(())
}
