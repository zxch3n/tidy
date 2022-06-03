extern crate wasm_bindgen;
use tidy_tree::{geometry::Coord, TidyTree, NULL_ID};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tidy(TidyTree);

#[wasm_bindgen]
impl Tidy {
    pub fn null_id() -> usize {
        NULL_ID
    }

    pub fn with_basic_layout() -> Self {
        Tidy(TidyTree::with_basic_layout())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add_node(&mut self, id: usize, width: Coord, height: Coord, parent_id: usize) {
        self.0.add_node(id, width, height, parent_id);
    }

    pub fn data(&mut self, id: &[usize], width: &[Coord], height: &[Coord], parent_id: &[usize]) {
        self.0.data(id, width, height, parent_id);
    }

    pub fn layout(&mut self) {
        self.0.layout();
    }

    pub fn get_pos(&self) -> Vec<Coord> {
        self.0.get_pos()
    }
}
