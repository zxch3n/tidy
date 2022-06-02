mod linked_list;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe {
        alert(&format!("Hello, {}!", name));
    }
}

#[wasm_bindgen]
pub fn sum_of_squares(input: &[i32]) -> i32 {
    input.iter().map(|x| x * x).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
