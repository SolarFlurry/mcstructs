use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct BlockType {
	pub namespace: String,
	pub states: Vec<(String, BlockState)>,
}

impl BlockType {
	pub fn new(namespace: &str) -> Self {
		BlockType {namespace: namespace.to_string(), states: vec![]}
	}
	pub fn set_state (mut self, state_name: &str, state: &BlockState) -> Self {
		for (name, value) in &mut self.states {
			if name == state_name {
				*name = state_name.to_string();
				*value = state.clone();
			}
		}
		self
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BlockState {
	String(String),
	Int(i32),
	Bool(u8),
}