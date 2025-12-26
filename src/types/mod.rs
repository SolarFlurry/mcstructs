use crate::structure::MCStructure;

pub struct Vec3<T> {
	e: [T; 3],
}

impl<T> Vec3<T> {
	
	pub fn new (x: T, y: T, z: T) -> Vec3<T> {
		Vec3{e: [x, y, z]}
	}

	pub fn x (&self) -> &T { &self.e[0] }
	pub fn y (&self) -> &T { &self.e[1] }
	pub fn z (&self) -> &T { &self.e[2] }
}

impl Vec3<i32> {
	pub const _000: Vec3<i32> = Vec3 {e: [0, 0, 0]};
}

pub struct BlockType {
	pub type_id: String,
	pub states: Vec<(String, BlockState)>,
}

impl BlockType {
	pub fn new(namespace: &str) -> Self {
		BlockType {type_id: namespace.to_string(), states: vec![]}
	}
	pub fn set_state (mut self, state_name: &str, state: &BlockState) -> Self {
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

struct Block<'a> {
	permutation: BlockType,
	structure: &'a MCStructure,
	index: u32,
}

impl<'a> Block<'a> {
	fn set_item_slot (mut self, item_type_id: &str, count: u8) -> Self {
		if !self.permutation.is_container() {
			panic!("the permutation is not a container");
		}
		let mut index_in_data = None::<usize>;
		for (i, (j, data)) in self.structure.block_position_data.iter().enumerate() {
			if *j == self.index {
				index_in_data = Some(i);
				break;
			}
		}
		if let Some(index_in_data) = index_in_data {

		}
		self
	}
}

#[derive(Clone)]
pub enum BlockState {
	String(String),
	Int(i32),
	Bool(u8),
}