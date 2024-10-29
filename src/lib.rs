mod utils;

use wasm_bindgen::prelude::*;
use strsim::normalized_levenshtein;


cfg_if::cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm-alert!");
}

#[wasm_bindgen]
pub fn compare_strings(s1: &str, s2: &str) -> f64 {
    normalized_levenshtein(s1, s2)
}