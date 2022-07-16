use crate::cell::Cell;

pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

pub struct Snake {
  cols: i32,
  rows: i32,
  body: Vec<Cell>,
  direction: Direction
}

impl Snake {
  pub fn new(cols: i32, rows: i32, init_cord: &Cell, init_dir: Direction) -> Self {
    let mut body: Vec<Cell> = Vec::new();

    for i in init_cord.get_col()..(init_cord.get_col() + 3) {
      body.push(Cell::new(i, init_cord.get_row()));
    }

    Self { cols, rows, body, direction: init_dir }
  }

  pub fn go(&mut self, on_food: bool) {
    if !on_food {
      self.body.pop();
    } 
    let head: &Cell = self.body.first().unwrap();
    let new_cell: Cell;
    match self.direction {
      Direction::Up => {
        if head.get_row() == 0 {
          new_cell = Cell::new(head.get_col(), self.rows - 1);
        } else {
          new_cell = Cell::new(head.get_col(), head.get_row() - 1);
        }
      }
      Direction::Down => {
        if head.get_row() == self.rows - 1 {
          new_cell = Cell::new(head.get_col(), 0);
        } else {
          new_cell = Cell::new(head.get_col(), head.get_row() + 1);
        }
      }
      Direction::Left => {
        if head.get_col() == 0 {
          new_cell = Cell::new(self.cols - 1, head.get_row());
        } else {
          new_cell = Cell::new(head.get_col() - 1, head.get_row());
        }

      },
      Direction::Right => {
        if head.get_col() == self.cols - 1 {
          new_cell = Cell::new(0, head.get_row());
        } else {
          new_cell = Cell::new(head.get_col() + 1, head.get_row());
        }
      },
    }
    self.body.insert(0, new_cell);
  }

  pub fn turn(&mut self, direction: Direction) {
    self.direction = direction;
  }

  pub fn has(&self, cell: &Cell) -> bool {
    for snake_cell in &self.body {
      if snake_cell.get_col() == cell.get_col() && 
        snake_cell.get_row() == cell.get_row() {
        return true;
      }
    }
    return false;
  }

  pub fn eat_self(&self) -> bool {
    let head: &Cell = self.body.first().unwrap();
    let mut count = 0;
    for cell in &self.body {
      if cell.get_col() == head.get_col() &&
        cell.get_row() == head.get_row() {
        count += 1;
      }
    }
    return count != 1;
  }

  pub fn get_body(&self) -> &Vec<Cell> {
    return &self.body;
  }
}