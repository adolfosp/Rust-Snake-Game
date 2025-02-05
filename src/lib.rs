use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

//Diminuir o tamanho do binário
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct Snake {
  body: Vec<usize>,
}

//wasmbindgen é uma macro que permite a comunicação entre o código Rust e o JavaScript
#[wasm_bindgen]
 pub struct World {
   pub width: usize,
 }

 #[wasm_bindgen]
 impl World {
  pub fn new() -> World {
    World { width : 16}
  }

  pub fn width(&self) -> usize {
    self.width
  }
 }