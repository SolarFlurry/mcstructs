use crate::{
    nbt::{TagData, TagKind, TagList},
    structure::MCStructure,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Vec3<T> {
    e: [T; 3],
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> &T {
        &self.e[0]
    }
    pub fn y(&self) -> &T {
        &self.e[1]
    }
    pub fn z(&self) -> &T {
        &self.e[2]
    }
}

impl Vec3<i32> {
    pub const _000: Vec3<i32> = Vec3 { e: [0, 0, 0] };
}

#[derive(Clone, Debug)]
pub struct BlockType {
    pub type_id: String,
    pub states: Vec<(String, BlockState)>,
}

impl BlockType {
    pub fn new(namespace: &str) -> Self {
        BlockType {
            type_id: namespace.to_string(),
            states: vec![],
        }
    }
    pub fn set_state(mut self, state_name: &str, state: &BlockState) -> Self {
        let mut state_idx: Option<usize> = None;
        for (idx, state) in &mut self.states.iter().enumerate() {
            if state.0 == state_name {
                state_idx = Some(idx)
            }
        }
        if let Some(idx) = state_idx {
            self.states[idx].1 = state.clone();
        } else {
            self.states.push((state_name.to_string(), state.clone()))
        }
        self
    }
    pub fn is_container(&self) -> bool {
        self.type_id == "minecraft:barrel" || self.type_id == "minecraft:chest"
    }
}

pub struct Block<'a> {
    permutation: BlockType,
    structure: &'a mut MCStructure,
    pub(in crate) index: u32,
}

pub(crate) fn set_item_slot_of_block(
    structure: &mut MCStructure,
    permutation: &BlockType,
    index: u32,
    slot: u8,
    item_type_id: &str,
    count: u8,
) {
    if !permutation.is_container() {
        panic!("the permutation is not a container");
    }
    let mut index_in_data = None::<usize>;
    for (i, (j, _data)) in structure.block_position_data.iter().enumerate() {
        if *j == index {
            index_in_data = Some(i);
            break;
        }
    }
	if let None = index_in_data {
		index_in_data = Some(structure.block_position_data.len());
		structure.block_position_data.push((index, TagData::Compound(TagList::new())))
	}
    if let Some(index_in_data) = index_in_data {
        let actual_data = &mut structure.block_position_data[index_in_data].1;
        let items: &mut TagData;
        if let Some(get) = actual_data.get_tag("Items") {
            items = get;
        } else {
            actual_data.add_tag("Items", TagData::List(TagKind::Compound, 0, vec![]));
            items = actual_data.get_tag("Items").expect("unreachable code");
        }

        if let TagData::List(_kind, size, list) = items {
            list.push(TagData::Compound(TagList::from(vec![
                ("Count".to_string(), TagData::Byte(count as i8)),
                (
                    "Name".to_string(),
                    TagData::String(item_type_id.to_string()),
                ),
                ("Slot".to_string(), TagData::Byte(slot as i8)),
            ])));
            *size += 1;
        }
    } else {
        panic!("unreachable code")
    }
}

impl<'a> Block<'a> {
    pub fn new(permutation: BlockType, index: u32, structure: &'a mut MCStructure) -> Block<'a> {
        Block {
            permutation,
            index,
            structure,
        }
    }
    pub fn set_item_slot(self, slot: u8, item_type_id: &str, count: u8) -> Self {
        set_item_slot_of_block(
            self.structure,
            &self.permutation,
            self.index,
            slot,
            item_type_id,
            count,
        );
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "tag", content = "contents")]
pub enum BlockState {
    String(String),
    Int(i32),
    Bool(u8),
}
