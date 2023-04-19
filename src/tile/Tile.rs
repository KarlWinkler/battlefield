use std::vec;

pub struct Tile {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  tile_vertices: Vec::<Vec<f32>>,
  index: u32,
  selected: bool,
}

impl Tile {
  pub fn new(x: f32, y: f32, width: f32, height: f32, index: u32) -> Tile {
    Tile {
      x,
      y,
      width,
      height,
      tile_vertices: Self::define_vertices(x, y, width, height),
      index,
      selected: false
    }
  }

  pub fn draw(&mut self, vertices: &mut Vec<f32>) {
    self.add_hexagon(vertices);
  }

  fn add_hexagon(&mut self, vertices: &mut Vec<f32>) {
    for i in 0..6 {
        // center point
        vertices.push(self.x);
        vertices.push(self.y);
        vertices.push(0.0);

        // first point
        vertices.push(self.tile_vertices[i % 6][0]);
        vertices.push(self.tile_vertices[i % 6][1]);
        vertices.push(0.0);

        // second point
        vertices.push(self.tile_vertices[(i + 1) % 6][0]);
        vertices.push(self.tile_vertices[(i + 1) % 6][1]);
        vertices.push(0.0);
    }
  }

  // https://www.geeksforgeeks.org/check-whether-a-given-point-lies-inside-a-triangle-or-not/
  pub fn hovered(&self, x_mouse: f32, y_mouse: f32) -> bool {
    // check first if the mouse is within the first triangle
    let p_mouse = vec![x_mouse, y_mouse];
    let p3 = vec![self.x, self.y];

    for i in 0..5 {
      let p1 = &self.tile_vertices[i];
      let p2 = &self.tile_vertices[i + 1];

      if self.is_in_triangle(&p1, &p2, &p3, &p_mouse) {
        log::info!("test: {}", self.is_in_triangle(&vec![1.0, 0.0], &vec![5.0, 1.0], &vec![3.0, 6.0], &vec![2.0, 5.0]));

        return true;
      }
    }

    false
  }

  pub fn toggle_selected(&mut self) {
    self.selected = !self.selected;

    if self.selected {
      self.expand();
    } else {
      self.contract();
    }

    self.tile_vertices = Self::define_vertices(self.x, self.y, self.width, self.height);
  }

  fn expand(&mut self) {
    self.height *= 1.1;
    self.width *= 1.1;
  }

  fn contract(&mut self) {
    self.height /= 1.1;
    self.width /= 1.1;
  }

  // calculate area of triangle using cross product
  fn area(p1: &Vec<f32>, p2: &Vec<f32>, p3: &Vec<f32>) -> f32 {
    let a = 0.5 * ((p2[0] - p1[0]) * (p3[1] - p1[1]) - (p3[0] - p1[0]) * (p2[1] - p1[1]));

    a.abs()
  }

  fn is_in_triangle(&self, p1: &Vec<f32>, p2: &Vec<f32>, p3: &Vec<f32>, p: &Vec<f32>) -> bool {
    
    let area = Self::area(p1, p2, p3);

    let a1 = Self::area(p1, p2, p);
    let a2 = Self::area(p2, p3, p);
    let a3 = Self::area(p3, p1, p);

    let a = a1 + a2 + a3;

    let a_delta = (area - a).abs(); 

    if a_delta < 0.001 {
      true
    }
    else {
      false
    }
  }

  fn define_vertices(x: f32, y: f32, width: f32, height: f32) -> Vec<Vec<f32>> {
    let mut tile_vertices = Vec::<Vec<f32>>::new();

    let mut angle = 0.0;
    let mut x1 = x + width;
    let mut y1 = y;

    for _ in 0..6 {
      let x2 = x + width * ((angle + 60.0) as f32).to_radians().cos();
      let y2 = y + height * ((angle + 60.0) as f32).to_radians().sin();

      tile_vertices.push(vec![x1, y1]);

      x1 = x2;
      y1 = y2;
      angle += 60.0;
    }

    tile_vertices
  }

  pub fn get_x(&self) -> f32 {
    self.x
  }

  pub fn get_y(&self) -> f32 {
    self.y
  }

  pub fn get_selected(&self) -> bool {
    self.selected
  }
}