//use serde_wasm_bindgen;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log_jsValue(s: &JsValue);
}

use crate::{structure::MCStructure, types::{BlockState, BlockType}};
use crate::types::Vec3;

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
	pub fn setblock(&mut self, loc: &[i32], block: WASM_BlockType) -> Result<(), JsValue> {
		let loc = vec3_from_slice(loc);
		
		self.mcstructure.setblock(loc, block.blocktype.expect("unreachable code"));
		Ok(())
	}
	pub fn as_bytes(&self) -> Vec<u8> {
		self.mcstructure.as_bytes()
	}
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct WASM_BlockType {
	blocktype: Option<BlockType>
}

#[wasm_bindgen]
impl WASM_BlockType {
	pub fn new(namespace: &str) -> WASM_BlockType {
		WASM_BlockType {
			blocktype: Some(BlockType::new(namespace))
		}
	}
	pub fn set_state(&mut self, state_name: &str, state_js: JsValue) -> Result<(), JsValue> {
		let state: BlockState =
			serde_wasm_bindgen::from_value(state_js).map_err(|e| JsValue::from_str(&e.to_string()))?;

		let inner = self.blocktype.take().unwrap();

		self.blocktype = Some(inner.set_state(state_name, &state));

		Ok(())
	}
}