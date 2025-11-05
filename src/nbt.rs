use std::{
    env::args,
    fs::File,
    io::{self, Error, ErrorKind, Read},
};

fn next_byte(file: &mut File) -> io::Result<u8> {
    let mut buffer = [0 as u8];
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
        Err(Error::from(ErrorKind::UnexpectedEof))
    } else {
        Ok(buffer[0])
    }
}

fn to_tag_string(tag: u8) -> String {
    match tag {
        1 => "TAG_Byte",
        2 => "TAG_Short",
        3 => "TAG_Int",
        5 => "TAG_Float",
        4 => "TAG_Long",
        8 => "TAG_String",
        9 => "TAG_List",
        10 => "TAG_Compound",
        _ => "unknown_tag",
    }
    .to_string()
}

fn tag_parse_payload(file: &mut File, tag: u8, indent: u8) -> io::Result<()> {
    match tag {
        1 => tag_byte(file)?,
        2 => tag_short(file)?,
        3 => tag_int(file)?,
        4 => tag_long(file)?,
        5 => tag_float(file)?,
        8 => tag_string(file)?,
        9 => tag_list(file, indent)?,
        10 => tag_compound(file, indent)?,
        _ => {
            println!("Unknown tag 0x{tag:02X}");
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    Ok(())
}

fn tag_parse(file: &mut File, tag: u8, indent: u8) -> io::Result<()> {
    let size = u16::from_le_bytes([next_byte(file)?, next_byte(file)?]);

    if size > 0 {
        let mut data: Vec<u8> = vec![];

        for _i in 0..size {
            data.push(next_byte(file)?);
        }

        let tagname = String::from_utf8(data);
        match tagname {
            Err(_error) => return Err(Error::from(ErrorKind::InvalidData)),
            Ok(name) => {
                print!("{name}: ")
            }
        };
    };

    print!("{}", to_tag_string(tag));

    tag_parse_payload(file, tag, indent)?;
    Ok(())
}

fn tag_byte(file: &mut File) -> io::Result<()> {
    let data = next_byte(file)? as i8;
    println!(" = {data}");
    Ok(())
}
fn tag_short(file: &mut File) -> io::Result<()> {
    let bytes: [u8; 2] = [next_byte(file)?, next_byte(file)?];

    println!(" = {}", i16::from_le_bytes(bytes));

    Ok(())
}
fn tag_int(file: &mut File) -> io::Result<()> {
    println!(
        " = {}",
        i32::from_le_bytes([
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?
        ])
    );

    Ok(())
}
fn tag_long(file: &mut File) -> io::Result<()> {
    println!(
        " = {}",
        i64::from_le_bytes([
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
        ])
    );

    Ok(())
}

fn tag_float(file: &mut File) -> io::Result<()> {
    println!(
        " = {}",
        f32::from_le_bytes([
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
            next_byte(file)?,
        ])
    );

    Ok(())
}

fn tag_string(file: &mut File) -> io::Result<()> {
    let size = u16::from_le_bytes([next_byte(file)?, next_byte(file)?]);

    let mut data: Vec<u8> = vec![];

    for _i in 0..size {
        data.push(next_byte(file)?);
    }

    let string = String::from_utf8(data);
    match string {
        Err(_error) => return Err(Error::from(ErrorKind::InvalidData)),
        Ok(string) => {
            println!(" = {string}");
        }
    };

    Ok(())
}

fn tag_list(file: &mut File, indent: u8) -> io::Result<()> {
    let tag = next_byte(file)?;
    let size = u32::from_le_bytes([
        next_byte(file)?,
        next_byte(file)?,
        next_byte(file)?,
        next_byte(file)?,
    ]);

    println!(" of {}[{}] = [", to_tag_string(tag), size);

    for _i in 0..size {
        print!("{}", "  ".repeat((indent + 1) as usize));
        tag_parse_payload(file, tag, indent + 1)?;
    }
    println!("{}]", "  ".repeat(indent as usize));

    Ok(())
}

fn tag_compound(file: &mut File, indent: u8) -> io::Result<()> {
    println!(" = {{");
    loop {
        let tag = next_byte(file)?;

        if tag == 0 {
            println!("{}}}", "  ".repeat(indent as usize));
            break;
        }
        print!("{}", "  ".repeat((indent + 1) as usize));
        tag_parse(file, tag, indent + 1)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if let Some(path) = args.get(1) {
        let mut file = File::open(path)?;

        /*let header = i32::from_le_bytes([
            next_byte(&mut file)?,
            next_byte(&mut file)?,
            next_byte(&mut file)?,
            next_byte(&mut file)?,
        ]);
		if header == 3 {
			next_byte(&mut file)?;
			next_byte(&mut file)?;
			next_byte(&mut file)?;
			next_byte(&mut file)?;
		} else {
			file.seek(SeekFrom::Current(-4))?;
		};*/

        loop {
            let tag = next_byte(&mut file)?;
            tag_parse(&mut file, tag, 0)?;
            let result = next_byte(&mut file);
            if let Err(error) = result {
                if let ErrorKind::UnexpectedEof = error.kind() {
                    break;
                } else {
                    return Err(error);
                }
            }
        }
    } else {
        println!("Requires 1 path argument");
        return Err(Error::from(ErrorKind::InvalidInput));
    };

    Ok(())
}
