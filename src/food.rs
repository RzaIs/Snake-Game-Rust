use crate::{cell::Cell, snake::Snake};

use rand::{
  prelude::ThreadRng,
  Rng
};

pub struct Food {
  cols: i32,
  rows: i32,
  cell: Cell,
  rng: ThreadRng

}

impl Food {
  pub fn new(cols: i32, rows: i32) -> Self {
    let rng: ThreadRng = rand::thread_rng();
    let cell: Cell = Cell::new(0, 0);
    Self { cols, rows, cell, rng }
  }

  pub fn get_cell(&self) -> &Cell {
    return &self.cell;
  }

  pub fn generate(&mut self, snake: &Snake) {
    let mut food_col: i32 = { 
      let num: i32 = self.rng.gen();
      i32::abs(num % self.cols)
    };

    let mut food_row: i32 = {
      let num: i32 = self.rng.gen();
      i32::abs(num % self.rows)
    };

    let mut food: Cell = Cell::new(food_col, food_row);

    while snake.has(&food) {
      food_col = { 
        let num: i32 = self.rng.gen();
        i32::abs(num % self.cols)
      };
  
      food_row = {
        let num: i32 = self.rng.gen();
        i32::abs(num % self.rows)
      };

      food = Cell::new(food_col, food_row);
    }

    self.cell = food;
  }
}