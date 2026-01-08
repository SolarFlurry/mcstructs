use std::cell::RefCell;
use std::rc::Rc;

//use serde_wasm_bindgen;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_jsValue(s: &JsValue);
}

use crate::types::{Vec3, set_item_slot_of_block};
use crate::{
    structure::MCStructure,
    types::{BlockState, BlockType},
};

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
    mcstructure: Rc<RefCell<MCStructure>>,
}

#[wasm_bindgen]
impl WASM_MCStructure {
    pub fn new(size: &[i32]) -> Self {
        let size = vec3_from_slice(size);
        let structure = MCStructure::new(size);
        WASM_MCStructure {
            mcstructure: Rc::new(RefCell::new(structure)),
        }
    }
    pub fn setblock(&mut self, loc: &[i32], block: WASM_BlockType) -> Result<WASM_Block, JsValue> {
        let loc = vec3_from_slice(loc);

        self.mcstructure
            .borrow_mut()
            .setblock(loc, block.blocktype.clone().expect("unreachable code"));
        Ok(WASM_Block::new(
            block,
            (self.mcstructure.borrow().size.z() * self.mcstructure.borrow().size.y() * loc.x()
                + self.mcstructure.borrow().size.z() * loc.y()
                + loc.z()) as u32,
            self,
        ))
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.mcstructure.borrow().as_bytes()
    }
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct WASM_BlockType {
    blocktype: Option<BlockType>,
}

#[wasm_bindgen]
impl WASM_BlockType {
    pub fn new(namespace: &str) -> WASM_BlockType {
        WASM_BlockType {
            blocktype: Some(BlockType::new(namespace)),
        }
    }
    pub fn set_state(&mut self, state_name: &str, state_js: JsValue) -> Result<(), JsValue> {
        let state: BlockState = serde_wasm_bindgen::from_value(state_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let inner = self.blocktype.take().unwrap();

        self.blocktype = Some(inner.set_state(state_name, &state));

        Ok(())
    }
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct WASM_Block {
    permutation: WASM_BlockType,
    structure: Rc<RefCell<MCStructure>>,
    index: u32,
}

#[wasm_bindgen]
impl WASM_Block {
    pub fn new(permutation: WASM_BlockType, index: u32, structure: &WASM_MCStructure) -> Self {
        WASM_Block {
            permutation,
            structure: structure.mcstructure.clone(),
            index,
        }
    }
    pub fn set_item_slot(&mut self, slot: u8, item_type_id: &str, count: u8) {
        set_item_slot_of_block(
            &mut self.structure.borrow_mut(),
            self.permutation
                .blocktype
                .as_ref()
                .expect("unreachable code"),
            self.index,
            slot,
            item_type_id,
            count,
        );
    }
}
