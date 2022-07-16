pub struct Cell {
  col: i32,
  row: i32,
}

impl Cell {
  pub fn new(col: i32, row: i32) -> Self{
    Self { col, row }
  }

  pub fn get_col(&self) -> i32 {
    return self.col;
  }

  pub fn get_row(&self) -> i32 {
    return self.row;
  }
}