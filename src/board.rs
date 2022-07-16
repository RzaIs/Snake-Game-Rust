use crate::cell::Cell;

pub struct Board {
  table: Vec<Vec<Cell>>,
}

impl Board {
  pub fn new(cols: i32, rows: i32) -> Self {
    let mut table: Vec<Vec<Cell>> = Vec::new();

    for i in 0..cols {
      let mut sub_vec: Vec<Cell> = Vec::new(); 
      for j in 0..rows {
        sub_vec.push(Cell::new(i, j));
      }
      table.push(sub_vec);
    }

    Self { table }
  }

  pub fn get_table(&self) -> &Vec<Vec<Cell>> {
    return &self.table;
  }
}