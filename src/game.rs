use std::time::Instant;

use sdl2::{ 
  Sdl,
  EventPump,
  event::Event, keyboard::Keycode
};

use crate::{
  cell::Cell,
  board::Board,
  food::Food,
  screen::{
    Screen,
    CellType
  },
  snake::{
    Snake,
    Direction
  }
};

pub struct Game {
  board: Board,
  snake: Snake,
  screen: Screen,
  food: Food,
  event_pump : EventPump,
}

impl Game {
  pub fn new(cols : i32, rows : i32, scale : i32) -> Self {

    let sdl_init: Sdl = sdl2::init().unwrap();
    
    let board: Board = Board::new(cols, rows);

    let snake: Snake = Snake::new(
      cols, rows, 
      &Cell::new(cols / 2, rows / 2),
      Direction::Left
    );

    let screen: Screen = Screen::new(cols, rows, scale, &sdl_init);

    let mut food: Food = Food::new(cols, rows);
    food.generate(&snake);

    let event_pump: EventPump = sdl_init.event_pump().unwrap();

    Self { board, snake, screen, food, event_pump }
  }

  pub fn start(&mut self) {
    
    let mut running: bool = true;
    let mut begin = Instant::now();
    let mut now: Instant;

    while running {
      for sub_vec in self.board.get_table() {
        for cell in sub_vec {
          self.screen.render(
            cell.get_col(),
            cell.get_row(),
            &CellType::Map
          );
        }
      }

      self.screen.render(
        self.food.get_cell().get_col(),
        self.food.get_cell().get_row(),
        &CellType::Food
      );

      let mut cell_type: CellType = CellType::SnakeDark; 
      for cell in self.snake.get_body() {
        self.screen.render(
          cell.get_col(),
          cell.get_row(),
          &cell_type
        );

        match cell_type {
          CellType::SnakeDark => cell_type = CellType::SnakeLight,
          CellType::SnakeLight => cell_type = CellType::SnakeDark,
          _ => {}
        }
      }

      self.screen.display();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => running = false,
          Event::KeyDown { keycode: Some(Keycode::Up), .. } => self.snake.turn(Direction::Up),
          Event::KeyDown { keycode: Some(Keycode::Down), .. } => self.snake.turn(Direction::Down),
          Event::KeyDown { keycode: Some(Keycode::Left), .. } => self.snake.turn(Direction::Left),
          Event::KeyDown { keycode: Some(Keycode::Right), .. } => self.snake.turn(Direction::Right),
          _ => {}
        }
      }

      now = Instant::now();

      if now.duration_since(begin).as_millis() > 512 {
        begin = Instant::now();
        if self.snake.has(self.food.get_cell()) {
          self.food.generate(&self.snake);
          self.snake.go(true);
        } else {
          self.snake.go(false);
        }
        if self.snake.eat_self() {
          running = false;
        }
      }
    }
  }
}