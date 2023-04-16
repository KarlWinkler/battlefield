#[path = "./tile/Tile.rs"]
mod tile;

pub struct Grid {
  width: u32,
  height: u32,

  cells: Vec::<tile::Tile>,
  vertices: Vec::<f32>,
  idxs: Vec::<u16>,
}

impl Grid {
  pub fn new(width: u32, height: u32) -> Grid {
    Grid {
      width: width,
      height: height,

      cells: Self::populate_cells(width, height),
      vertices: Vec::<f32>::new(),
      idxs: Vec::<u16>::new()
    }
  }

  fn populate_cells(width: u32, height: u32) -> Vec::<tile::Tile> {
    let mut cells = Vec::<tile::Tile>::new();

    let radius = 1.0/height as f32;
    let diameter = radius * 2.0;
    let y_origin = 1.0 - radius * 5.0/2.0;
    let x_origin = -1.0 + radius;
    let ratio = 0.5;

    for i in 0..width - 1 {
      for j in 0..height - 1 {

        let y = y_origin - (j as f32) * diameter + radius * ((i % 2) as f32) ;
        let x = x_origin + (i as f32) * ratio * diameter;

        let tile = tile::Tile::new(x, y, radius * ratio, radius);
        cells.push(tile);
      }
    }

    cells
  }

  pub fn draw(&mut self) {
    self.vertices.clear();

    for i in &mut self.cells {
      i.draw(&mut self.vertices);
    }
  }

  pub fn get_vertices(&self) -> &Vec::<f32> {
    &self.vertices
  }

  pub fn get_hex_hovered(&self, x: f32, y: f32) -> Option<u32> {
    let mut idx = None;

    for (i, cell) in self.cells.iter().enumerate() {
      if cell.hovered(x, y) {
        idx = Some(i as u32);
      }
    }

    idx
  }
}