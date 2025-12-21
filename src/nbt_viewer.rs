use std::{
    env::args,
    fs::File,
    io::{self, Error, ErrorKind, Read},
};

use mcstructs::nbt::NbtTree;

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if let Some(path) = args.get(1) {
        let mut file = File::open(path)?;

        let mut bytes = Vec::<u8>::new();
        let mut buffer = [0_u8; 1];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            bytes.extend(&buffer);
        }
	    let nbt = NbtTree::from_bytes(args.get(2).is_some(), bytes);
	    nbt.print();
    } else {
        println!("Requires 1 path argument");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    Ok(())
}
