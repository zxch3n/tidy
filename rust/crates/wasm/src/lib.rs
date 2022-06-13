extern crate wasm_bindgen;
use tidy_tree::{geometry::Coord, LayoutType, TidyTree, NULL_ID};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tidy(TidyTree);

#[wasm_bindgen]
pub enum WasmLayoutType {
    Basic,
    Tidy,
    LayeredTidy,
}

#[wasm_bindgen]
impl Tidy {
    pub fn null_id() -> usize {
        NULL_ID
    }

    pub fn with_basic_layout() -> Self {
        Tidy(TidyTree::with_basic_layout())
    }

    pub fn with_tidy_layout() -> Self {
        Tidy(TidyTree::with_tidy_layout())
    }

    pub fn with_layered_tidy() -> Self {
        Tidy(TidyTree::with_layered_tidy())
    }

    pub fn change_layout(&mut self, layout_type: WasmLayoutType) {
        match layout_type {
            WasmLayoutType::Basic => self.0.change_layout(LayoutType::Basic),
            WasmLayoutType::Tidy => self.0.change_layout(LayoutType::Tidy),
            WasmLayoutType::LayeredTidy => self.0.change_layout(LayoutType::LayeredTidy),
        }
    }

    pub fn remove_node(&mut self, id: usize) {
        self.0.remove_node(id);
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
