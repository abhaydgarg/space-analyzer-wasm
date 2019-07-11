use wasm_bindgen::prelude::*;
use serde_json;

mod timer;
mod scanner;

#[wasm_bindgen]
pub fn scan(path: &str) -> String {
  match scanner::run(path) {
    Ok(r) => {
      let json = serde_json::to_string(&r).unwrap();
      return json;
    }
    Err(e) => {
      return e.to_string();
    }
  };
}
