use std::{fs::File, io::{self, Write}};

use mcstructs::{structure::MCStructure, types::{BlockState, BlockType, Vec3}};

fn main () -> io::Result<()> {let path = "generated.mcstructure";

	let mut structure = MCStructure::new(Vec3::<i32>::new(1, 2, 1));

	structure.setblock(Vec3::<i32>::_000, 
		BlockType::new("minecraft:grass".to_string())
		.set_state(&"minecraft:cardinal_direction".to_string(), &BlockState::String("north".to_string()))
		.set_state(&"output_lit_bit".to_string(), &BlockState::Bool(0))
		.set_state(&"output_subtract_bit".to_string(), &BlockState::Bool(1))
	);

	let mut file = File::create(path)?;
	let data = structure.as_bytes();

	file.write_all(&data)?;
	println!("Wrote {} bytes to '{}'", data.len(), path);

	Ok(())
}