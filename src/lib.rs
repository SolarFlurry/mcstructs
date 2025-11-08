use serde_wasm_bindgen;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

pub mod nbt;
pub mod structure;
pub mod types;

use crate::{structure::MCStructure, types::{BlockState, BlockType}};
use types::Vec3;

// MCStructure
#[wasm_bindgen]
pub fn mcstructure_new(data: JsValue) -> Result<JsValue, JsValue> {
    let size: Vec3<i32> =
        serde_wasm_bindgen::from_value(data).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let structure = MCStructure::new(size);

    serde_wasm_bindgen::to_value(&structure).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn mcstructure_setblock(self_js: JsValue, loc_js: JsValue, block_js: JsValue, ) -> Result<JsValue, JsValue> {
    let mut object: MCStructure =
        serde_wasm_bindgen::from_value(self_js).map_err(|e| JsValue::from_str(&e.to_string()))?;

	let loc: Vec3<i32> =
		serde_wasm_bindgen::from_value(loc_js).map_err(|e| JsValue::from_str(&e.to_string()))?;
	let block: BlockType =
		serde_wasm_bindgen::from_value(block_js).map_err(|e| JsValue::from_str(&e.to_string()))?;

	object.setblock(loc, block);

	serde_wasm_bindgen::to_value(&object).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn mcstructure_as_bytes (self_js: JsValue) -> Result<JsValue, JsValue> {
	let object: MCStructure =
        serde_wasm_bindgen::from_value(self_js).map_err(|e| JsValue::from_str(&e.to_string()))?;

	serde_wasm_bindgen::to_value(&object.as_bytes()).map_err(|e| JsValue::from_str(&e.to_string()))
}

// Vec3
#[wasm_bindgen]
pub fn vec3_i32_new (x: i32, y: i32, z: i32) -> Result<JsValue, JsValue> {
	serde_wasm_bindgen::to_value(&Vec3::<i32>::new(x,y,z)).map_err(|e| JsValue::from_str(&e.to_string()))
}

// BlockType
#[wasm_bindgen]
pub fn blocktype_new (namespace: &str) -> Result<JsValue, JsValue> {
	serde_wasm_bindgen::to_value(&BlockType::new(namespace)).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn blocktype_set_state (self_js: JsValue, state_name: &str, state_js: JsValue) -> Result<JsValue, JsValue> {
	let object: BlockType =
		serde_wasm_bindgen::from_value(self_js).map_err(|e| JsValue::from_str(&e.to_string()))?;

	let state: BlockState =
		serde_wasm_bindgen::from_value(state_js).map_err(|e| JsValue::from_str(&e.to_string()))?;
	
	let object = object.set_state(state_name, &state);

	serde_wasm_bindgen::to_value(&object).map_err(|e| JsValue::from_str(&e.to_string()))
}