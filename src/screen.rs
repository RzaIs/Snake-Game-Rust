use sdl2::{
  video::Window,
  render::Canvas,
  pixels::Color,
  rect::Rect,
  Sdl
};

pub enum CellType {
  SnakeLight, SnakeDark, Food, Map
}

pub struct Screen {
  canvas : Canvas<Window>,
  scale : i32
}

impl Screen {
  pub fn new(cols : i32, rows : i32, scale : i32, sdl_init: &Sdl) -> Self{
    let window: Window = sdl_init
      .video()
      .unwrap()
      .window(
        "Game of Life",
        (cols * scale).try_into().unwrap(),
        (rows * scale).try_into().unwrap()
      ).position_centered()
      .build()
      .unwrap();

    let canvas: Canvas<Window> = window
      .into_canvas()
      .build()
      .unwrap();

    Self { canvas, scale }
  }


  pub fn render(&mut self, col: i32, row: i32, cell_type: &CellType) {
    let rect: Rect = Rect::new(
      col * self.scale, 
      row * self.scale,
      self.scale.try_into().unwrap(),
      self.scale.try_into().unwrap()
    );

    match cell_type {
      CellType::Food => self.canvas.set_draw_color(Color::RGB(255, 0, 0)),
      CellType::SnakeLight => self.canvas.set_draw_color(Color::RGB(0, 255, 0)),
      CellType::SnakeDark => self.canvas.set_draw_color(Color::RGB(0, 156, 0)),
      CellType::Map => {
        if (col + row) % 2 == 1 {
          self.canvas.set_draw_color(Color::RGB(60, 60, 60));
        } else {
          self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        }
      }
    }

    let result: Result<(), String> = self.canvas.fill_rect(rect);
    if result.is_err() {
      print!("{}", sdl2::get_error());
    }
  }

  pub fn display(&mut self) {
    self.canvas.present();
  }
}