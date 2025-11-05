use core::panic;

use crate::types::{BlockType, Vec3};

pub struct MCStructure {
	size: Vec3<i32>,
	blocks: Vec<i32>,
	palette: Vec<BlockType>,
}

#[repr(u8)]
#[allow(dead_code)]
enum TAG {
	End = 0,
	Byte,
	Short,
	Int,
	Long,
	Float,
	Double,
	ByteArray,
	String,
	List,
	Compound,
	IntArray,
	LongArray,
}

fn write_string(buffer: &mut Vec<u8>, string: &str) {
	buffer.extend(u16::to_le_bytes(string.len() as u16));
	buffer.extend_from_slice(string.as_bytes());
}

impl MCStructure {
	pub fn new (size: Vec3<i32>) -> MCStructure {
		let mut blocks: Vec<i32> = vec![];
		for _i in 0..(size.x()*size.y()*size.z()) {
			blocks.push(-1);
		}
		MCStructure {size, blocks, palette: vec![]}
	}
	pub fn setblock (&mut self, loc: Vec3<i32>, block: BlockType) {
		if loc.x() >= self.size.x() || loc.y() >= self.size.y() || loc.z() >= self.size.z() {
			panic!("Location specified is out of structure bounds");
		}
		let index = self.size.z()*self.size.y()*loc.x() + self.size.z()*loc.y() + loc.z();
		self.blocks[index as usize] = self.palette.len() as i32;
		self.palette.push(block);
	}

	pub fn as_bytes (&self) -> Vec<u8> {
		let mut bytes: Vec<u8> = vec![10, 0, 0];

		// format_version
		bytes.push(TAG::Int as u8);
		write_string(&mut bytes, "format_version");
		bytes.extend(i32::to_le_bytes(1));

		// size
		bytes.push(TAG::List as u8);
		write_string(&mut bytes, "size");
		bytes.push(TAG::Int as u8);
		bytes.extend(u32::to_le_bytes(3));
		bytes.extend(i32::to_le_bytes(*self.size.x()));
		bytes.extend(i32::to_le_bytes(*self.size.y()));
		bytes.extend(i32::to_le_bytes(*self.size.z()));

		// block_indices
		bytes.push(TAG::List as u8);
		write_string(&mut bytes, "block_indices");
		bytes.push(TAG::List as u8);
		bytes.extend(u32::to_le_bytes(2));

		// first layer
		bytes.push(TAG::Int as u8);
		bytes.extend(u32::to_le_bytes(self.blocks.len() as u32));
		for value in &self.blocks {
			bytes.extend(i32::to_le_bytes(*value));
		}
		// second layer
		bytes.push(TAG::Int as u8);
		bytes.extend(u32::to_le_bytes(self.blocks.len() as u32));
		for _i in 0..self.blocks.len() {
			bytes.extend(i32::to_le_bytes(-1));
		}

		// palette
		bytes.push(TAG::Compound as u8);
		write_string(&mut bytes, "palette");
		bytes.push(TAG::Compound as u8);
		write_string(&mut bytes, "default");
		bytes.push(TAG::List as u8);
		write_string(&mut bytes, "block_palette");
		bytes.push(TAG::Compound as u8);
		bytes.extend(u32::to_le_bytes(self.palette.len() as u32));
		for value in &self.palette {
			bytes.push(TAG::String as u8);
			write_string(&mut bytes, "name");
			write_string(&mut bytes, &value.namespace);
			bytes.push(0);
		}
		bytes.push(0);
		bytes.push(0);

		// structure_world_origin
		bytes.push(TAG::List as u8);
		write_string(&mut bytes, "structure_world_origin");
		bytes.push(TAG::Int as u8);
		bytes.extend(u32::to_le_bytes(3));
		bytes.extend(i32::to_le_bytes(0));
		bytes.extend(i32::to_le_bytes(0));
		bytes.extend(i32::to_le_bytes(0));

		bytes.push(0);

		bytes
	}
}