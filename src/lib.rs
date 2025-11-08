use serde_wasm_bindgen;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

pub mod nbt;
pub mod structure;
pub mod types;

use crate::{structure::MCStructure, types::{BlockState, BlockType}};
use types::Vec3;

fn vec3_from_slice(data: &[i32]) -> Vec3<i32> {
	if data.len() != 3 {
		panic!("length of vec3 is not 3")
	}
	Vec3::<i32>::new(data[0], data[1], data[2])
}

// MCStructure
#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct WASM_MCStructure {
	mcstructure: MCStructure
}

#[wasm_bindgen]
impl WASM_MCStructure {
	pub fn new (size: &[i32]) -> Result<WASM_MCStructure, JsValue> {
		let size = vec3_from_slice(size);
		let structure = MCStructure::new(size);
		Ok(WASM_MCStructure {mcstructure: structure})
	}
	pub fn setblock(&mut self, loc: &[i32], block: JsValue) -> Result<(), JsValue> {
		let loc = vec3_from_slice(loc);
		let block: BlockType =
			serde_wasm_bindgen::from_value(block).map_err(|e| JsValue::from_str(&e.to_string()))?;
		
		self.mcstructure.setblock(loc, block);
		Ok(())
	}
	pub fn as_bytes(&self) -> Vec<u8> {
		self.mcstructure.as_bytes()
	}
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