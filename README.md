# MCStructs

A Rust crate and NPM package used for manipulating Minecraft Bedrock Edition structures and `.mcstructure` files.

JavaScript Example:
```js
import fs from "node:fs"
import {MCStructure, BlockType, Vec3} from "mcstructs";

let structure = new MCStructure(new Vec3(2, 2, 2));

structure.setBlock(new Vec3(0, 0, 0),
	new BlockType("minecraft:grass_block")
)

const data = Buffer.from(structure.asBytes())

fs.writeFile('example.mcstructure', data, (err) => {
	if (err) return;
	console.log(`Wrote ${data.lenght} bytes to 'generated.mcstructure'`)
})
```

Rust Example:
```rust
use std::{fs::File, io::{self, Write}};
use mcstructs::{structure::MCStructure, types::{BlockType, Vec3}};

fn main () -> io::Result<()> {
	let mut structure = MCStructure::new(Vec3::<i32>::new(2, 2, 2));

	structure.setblock(Vec3::<i32>::_000, 
		BlockType::new("minecraft:grass_block")
	);

	let mut file = File::create("example.mcstructure")?;

	let data = structure.as_bytes();
	file.write_all(&data)?;

	println!("Wrote {} bytes to 'generated.mcstructure'", data.len());

	Ok(())
}

```

There is an experimental JavaScript binding and wrapper in the branch `js-bindings`