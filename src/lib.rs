use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub category: String,
}

#[wasm_bindgen]
pub struct Oxide {
    db: Vec<Document>,
}

#[wasm_bindgen]
impl Oxide {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Oxide { db: Vec::new() }
    }

    // 1件追加して、最新の全データをJSに返して保存を促す
    pub fn insert_and_sync(&mut self, id: String, content: String, category: String) -> JsValue {
        self.db.push(Document { id, content, category });
        self.get_all()
    }

    pub fn get_all(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.db).unwrap()
    }

    pub fn load_from_js(&mut self, val: JsValue) {
        self.db = serde_wasm_bindgen::from_value(val).unwrap_or_default();
    }
}