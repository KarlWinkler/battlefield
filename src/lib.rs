// https://hackernoon.com/including-files-and-deeply-directories-in-rust-q35o3yer

mod utils;
mod webgl;

extern crate console_error_panic_hook;

use std::panic;
use wasm_bindgen::prelude::*;
// use std::fmt;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
  pub fn tick(&mut self) {
    let mut grid = webgl::grid::Grid::new(26, 13);
    let mut webgl_result = webgl::run(&mut grid);

    webgl_result = match webgl_result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
  }

  pub fn new(width: u32, height: u32) -> Universe {
      utils::set_logger();
      utils::set_panic_hook();

      panic::set_hook(Box::new(console_error_panic_hook::hook));

      let mut rng = rand::thread_rng();

      let cells = (0..width * (height / 8))
          .map(|_i| {
              rng.gen_range(0..255)
          }).collect::<Vec<u8>>();


      Universe {
          width,
          height,
          cells,
      }
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
      self.height
  }
}

impl Universe {
  
}


#[wasm_bindgen(start)]
fn main() {

  // webgl_result = match webgl_result {
  //     Ok(_) => Ok(()),
  //     Err(e) => Err(e),
  // };

  // print!("WebGL Result: {:?}", webgl_result);
}
