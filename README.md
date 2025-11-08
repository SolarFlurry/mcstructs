# MCStructs

A Rust crate used for manipulating Minecraft Bedrock Edition structures and `.mcstructure` files.

Example:
```rust
use std::{fs::File, io::{self, Write}};
use mcstructs::{structure::MCStructure, types::{BlockType, Vec3}};

fn main () -> io::Result<()> {
	let structure = MCStructure::new(Vec3::<i32>::new(2, 2, 2));

	structure.setblock(Vec3::<i32>::_000, 
		BlockType::new("minecraft:grass_block")
	);

	let mut file = File::create("example.mcstructure")?;

	let data = structure.as_bytes();
	file.write_all(&data)?;

	println!("Wrote {} bytes to '{}'", data.len(), path);

	Ok(())
}

```

There is an experimental JavaScript binding and wrapper in the branch `js-bindings`