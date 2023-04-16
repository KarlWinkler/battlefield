use std::vec;

pub struct Tile {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  tile_vertices: Vec::<Vec<f32>>,
}

impl Tile {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Tile {
    Tile {
      x,
      y,
      width,
      height,
      tile_vertices: Self::define_vertices(x, y, width, height),
    }
  }

  pub fn draw(&mut self, vertices: &mut Vec<f32>) {
    self.add_hexagon(vertices);
  }

  fn add_hexagon(&mut self, vertices: &mut Vec<f32>) {

    let vertices = vertices;
    let x = self.x;
    let y = self.y;
    let width = self.width;
    let height = self.height;

    let mut angle = 0.0;
    let mut x1 = x + width;
    let mut y1 = y;

    for _ in 0..6 {
        let x2 = x + width * ((angle + 60.0) as f32).to_radians().cos();
        let y2 = y + height * ((angle + 60.0) as f32).to_radians().sin();

        // center point
        vertices.push(x);
        vertices.push(y);
        vertices.push(0.0);

        // first point
        vertices.push(x1);
        vertices.push(y1);
        vertices.push(0.0);

        // second point
        vertices.push(x2);
        vertices.push(y2);
        vertices.push(0.0);

        x1 = x2;
        y1 = y2;
        angle += 60.0;
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

      if Self::is_in_triangle(&p1, &p2, &p3, &p_mouse) {
        return true;
      }
    }

    false
  }

  fn is_in_triangle(p1: &Vec<f32>, p2: &Vec<f32>, p3: &Vec<f32>, p: &Vec<f32>) -> bool {
    let denominator = (p2[1] - p3[1]) * (p1[0] - p3[0]) + (p3[0] - p2[0]) * (p1[1] - p3[1]);
    let a = (p2[1] - p3[1]) * (p[0] - p3[0]) + (p3[0] - p2[0]) * (p[1] - p3[1]) / denominator; 
    let b = (p3[1] - p1[1]) * (p[0] - p3[0]) + (p1[0] - p3[0]) * (p[1] - p3[1]) / denominator;
    let c = 1.0 - a - b;

    if a >= 0.0 && b >= 0.0 && c >= 0.0 { 
      true
    }
    else {
      false
    }
  }

  fn define_vertices(x: f32, y: f32, width: f32, height: f32) -> Vec<Vec<f32>> {
    let v1 = vec![x - width / 4.0, y + height / 2.0];
    let v2 = vec![x + width / 4.0, y + height / 2.0];
    let v3 = vec![x + width / 2.0, y];
    let v4 = vec![x + width / 4.0, y - height / 2.0];
    let v5 = vec![x - width / 4.0, y - height / 2.0];
    let v6 = vec![x - width / 2.0, y];

    vec![v1, v2, v3, v4, v5, v6]
  }
}