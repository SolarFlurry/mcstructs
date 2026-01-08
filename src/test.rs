use std::{fs::{self, File}, io::{self, Write}};

use mcstructs::{nbt::NbtTree, structure::MCStructure, types::{BlockState, BlockType, Vec3}};

fn main () -> io::Result<()> {
	let path = "generated.mcstructure";

	let mut structure = MCStructure::new(Vec3::<i32>::new(1, 2, 1));

	structure.setblock(Vec3::<i32>::_000, 
		BlockType::new("minecraft:unpowered_comparator")
		.set_state("minecraft:cardinal_direction", &BlockState::String("north".to_string()))
		.set_state("output_lit_bit", &BlockState::Bool(0))
		.set_state("output_subtract_bit", &BlockState::Bool(1))
	);

	structure.setblock(Vec3::new(0, 1, 0),
		BlockType::new("minecraft:barrel")
	).set_item_slot(0, "minecraft:redstone", 32);

	let mut file = File::create(path)?;
	let data = structure.as_bytes();

	file.write_all(&data)?;
	println!("Wrote {} bytes to '{}'", data.len(), path);

	let bytes = fs::read(path)?;
	let nbt = NbtTree::from_bytes(true, bytes);
	nbt.print();

	Ok(())
}