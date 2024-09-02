use {
  crate::{
    direction::Direction, progress::Progress, snake::Snake, BODY_SIZE_U32, CELL_SIZE_I32,
    HEAD_SIZE_U32, WINDOW_SIZE,
  },
  sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump, Sdl, VideoSubsystem,
  },
  std::{ops::ControlFlow, thread::sleep, time::Duration},
};

pub struct Front {
  canvas: Canvas<Window>,
  events: EventPump,
  direction: Option<Direction>,
}

impl Front {
  pub fn new() -> Result<Self, String> {
    let context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = context.video()?;
    let window: Window = video_subsystem
      .window("Snake", WINDOW_SIZE, WINDOW_SIZE)
      .position_centered()
      .build()
      .map_err(|err| err.to_string())?;
    let canvas: Canvas<Window> = window
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|err| err.to_string())?;
    let events: EventPump = context.event_pump()?;
    Ok(Self {
      canvas,
      events,
      direction: None,
    })
  }

  pub fn initial_present(&mut self) {
    self.canvas.set_draw_color(Color::RGB(20, 20, 20));
    self.canvas.clear();
    self.canvas.present();
  }

  pub fn r#loop(&mut self, snake: &mut Snake) {
    loop {
      let tick = self.tick(snake);
      if tick == ControlFlow::Break(()) {
        return;
      }
    }
  }

  fn tick(&mut self, snake: &mut Snake) -> ControlFlow<()> {
    self.direction = None;
    let cf = self.process_events();
    if cf == ControlFlow::Break(()) {
      return cf;
    }
    self.clear_screen();
    let progress: Progress = self.tick_game_once(snake);
    self.draw_game(snake);
    sleep(Duration::from_millis(100));
    match progress {
      Progress::Ongoing => ControlFlow::Continue(()),
      Progress::Won | Progress::Lost => ControlFlow::Break(()),
    }
  }

  fn clear_screen(&mut self) {
    self.canvas.set_draw_color(Color::RGB(20, 20, 20));
    self.canvas.clear();
  }

  fn process_events(&mut self) -> ControlFlow<()> {
    let events: Vec<Event> = self.events.poll_iter().collect();
    for event in events {
      match event {
        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        }
        | Event::Quit { .. } => return ControlFlow::Break(()),
        Event::KeyDown {
          keycode: Some(Keycode::W),
          ..
        } => {
          self.direction = Some(Direction::Up);
        },
        Event::KeyDown {
          keycode: Some(Keycode::A),
          ..
        } => {
          self.direction = Some(Direction::Left);
        },
        Event::KeyDown {
          keycode: Some(Keycode::S),
          ..
        } => {
          self.direction = Some(Direction::Down);
        },
        Event::KeyDown {
          keycode: Some(Keycode::D),
          ..
        } => {
          self.direction = Some(Direction::Right);
        },
        _ => {},
      }
    }
    ControlFlow::Continue(())
  }

  fn tick_game_once(&self, snake: &mut Snake) -> Progress {
    let _ = self;
    if let Some(dir) = &self.direction {
      snake.change_direction(dir);
    }
    snake.r#move()
  }

  #[expect(clippy::unwrap_used)]
  fn draw_game(&mut self, snake: &Snake) {
    // Draw Fruit
    self.canvas.set_draw_color(Color::RGB(100, 40, 40));
    self
      .canvas
      .fill_rect(Rect::new(
        snake.fruit.x as i32 * CELL_SIZE_I32 + 2,
        snake.fruit.y as i32 * CELL_SIZE_I32 + 2,
        BODY_SIZE_U32,
        BODY_SIZE_U32,
      ))
      .unwrap();
    // Draw Snake
    self.canvas.set_draw_color(Color::RGB(200, 200, 170));
    self
      .canvas
      .fill_rect(Rect::new(
        snake.cells[0].x as i32 * CELL_SIZE_I32 + 1,
        snake.cells[0].y as i32 * CELL_SIZE_I32 + 1,
        HEAD_SIZE_U32,
        HEAD_SIZE_U32,
      ))
      .unwrap();

    for cell in &snake.cells[1..] {
      self.canvas.set_draw_color(Color::RGB(200, 200, 170));
      self
        .canvas
        .fill_rect(Rect::new(
          cell.x as i32 * CELL_SIZE_I32 + 2,
          cell.y as i32 * CELL_SIZE_I32 + 2,
          BODY_SIZE_U32,
          BODY_SIZE_U32,
        ))
        .unwrap();
    }
    self.canvas.present();
  }
}
