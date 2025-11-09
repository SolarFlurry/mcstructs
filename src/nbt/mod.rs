#[repr(u8)]
#[derive(Clone, PartialEq, Debug)]
pub enum TagKind {
    End,
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

impl TagKind {
    pub fn from_u8(value: u8) -> TagKind {
        match value {
            0 => TagKind::End,
            1 => TagKind::Byte,
            2 => TagKind::Short,
            3 => TagKind::Int,
            4 => TagKind::Long,
            5 => TagKind::Float,
            6 => TagKind::Double,
            7 => TagKind::ByteArray,
            8 => TagKind::String,
            9 => TagKind::List,
            10 => TagKind::Compound,
            11 => TagKind::IntArray,
            12 => TagKind::LongArray,
            _ => panic!("Unknown tag 0x{:02X}", value),
        }
    }
}

fn next_byte(bytes: &[u8], ptr: &mut usize) -> u8 {
    let byte = bytes[*ptr];
    *ptr += 1;
    byte
}

pub struct NbtTree {
    entries: Vec<NbtTag>,
}

impl NbtTree {
    pub fn print(&self) {
        for tag in &self.entries {
            tag.print(0);
        }
    }
    pub fn new(entries: Vec<NbtTag>) -> Self {
        NbtTree { entries }
    }
    pub fn add_entry(&mut self, id: &str, data: TagData) {
        self.entries.push(NbtTag {
            id: id.to_string(),
            data,
        });
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        for tag in &self.entries {
            bytes.extend(tag.as_bytes());
        }
        bytes
    }
    pub fn from_bytes(bytes: &[u8]) -> NbtTree {
        let mut ptr = 0;
        let mut tree = NbtTree::new(vec![]);
        loop {
            let kind = TagKind::from_u8(next_byte(bytes, &mut ptr));
            tree.entries.push(NbtTag::from_bytes(bytes, &mut ptr, &kind));
            if ptr >= bytes.len() {
                break tree;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NbtTag {
    pub id: String,
    pub data: TagData,
}
impl NbtTag {
    pub fn print(&self, indent: usize) {
        print!("{}{}: ", "  ".repeat(indent), self.id);
        self.data.print(indent);
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.data.kind() as u8);
        bytes.extend(u16::to_le_bytes(self.id.len() as u16));
        bytes.extend_from_slice(self.id.as_bytes());
        bytes.extend(self.data.as_bytes());

        bytes
    }
    pub fn from_bytes(bytes: &[u8], ptr: &mut usize, kind: &TagKind) -> NbtTag {
        let id_size = u16::from_le_bytes([next_byte(bytes, ptr), next_byte(bytes, ptr)]);

        let mut id = Vec::<u8>::new();
        for _i in 0..id_size {
            id.push(next_byte(bytes, ptr));
        }
        let id = String::from_utf8(id);
        match id {
            Err(_err) => panic!("Invalid id name"),
            Ok(id) => {
                return NbtTag {
                    id,
                    data: TagData::from_bytes(bytes, ptr, &kind),
                };
            }
        };
    }
}

#[derive(Clone, Debug)]
pub enum TagData {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(i32, Vec<i8>),
    String(String),
    List(TagKind, u32, Vec<TagData>),
    Compound(Vec<NbtTag>),
    IntArray(i32, Vec<i32>),
    LongArray(i32, Vec<i64>),
}

impl TagData {
    pub fn print(&self, indent: usize) {
        match self {
            TagData::Byte(byte) => println!("\x1b[36mTAG_Byte\x1b[0m = \x1b[33m{}\x1b[0m", byte),
            TagData::Short(short) => println!("\x1b[36mTAG_Short\x1b[0m = \x1b[33m{}\x1b[0m", short),
            TagData::Int(int) => println!("\x1b[36mTAG_Int\x1b[0m = \x1b[33m{}\x1b[0m", int),
            TagData::Long(long) => println!("\x1b[36mTAG_Long\x1b[0m = \x1b[33m{}\x1b[0m", long),
            TagData::Float(float) => println!("\x1b[36mTAG_Float\x1b[0m = \x1b[33m{}\x1b[0m", float),
            TagData::Double(double) => println!("\x1b[36mTAG_Double\x1b[0m = \x1b[33m{}\x1b[0m", double),
            TagData::String(string) => println!("\x1b[36mTAG_String\x1b[0m = \x1b[32m'{}'\x1b[0m", string),
            TagData::List(_kind, _size, list) => {
                if list.len() == 0 {
                    println!("\x1b[36mTAG_List\x1b[0m = []");
                    return;
                }
                println!("\x1b[36mTAG_List\x1b[0m = [");
                for data in list {
                    print!("{}", "  ".repeat(indent + 1));
                    data.print(indent + 1)
                }
                println!("{}]", "  ".repeat(indent));
            }
            TagData::Compound(compound) => {
                if compound.len() == 0 {
                    println!("\x1b[36mTAG_Compound\x1b[0m = {{}}");
                    return;
                }
                println!("\x1b[36mTAG_Compound\x1b[0m = {{");
                for tag in compound {
                    tag.print(indent + 1);
                }
                println!("{}}}", "  ".repeat(indent));
            }
            _ => println!("Unknown Tag")
        }
    }
    pub fn kind(&self) -> TagKind {
        match self {
            TagData::End => TagKind::End,
            TagData::Byte(..) => TagKind::Byte,
            TagData::Short(..) => TagKind::Short,
            TagData::Int(..) => TagKind::Int,
            TagData::Long(..) => TagKind::Long,
            TagData::Float(..) => TagKind::Float,
            TagData::Double(..) => TagKind::Double,
            TagData::ByteArray(..) => TagKind::ByteArray,
            TagData::String(..) => TagKind::String,
            TagData::List(..) => TagKind::List,
            TagData::Compound(..) => TagKind::Compound,
            TagData::IntArray(..) => TagKind::IntArray,
            TagData::LongArray(..) => TagKind::LongArray,
        }
    }
    pub fn add_tag(&mut self, id: &str, data: TagData) {
        if let TagData::Compound(tags) = self {
            tags.push(NbtTag {
                id: id.to_string(),
                data,
            });
        } else {
            panic!("cannot use 'add_tag' on a non-compound tag");
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            TagData::Byte(byte) => i8::to_be_bytes(*byte).to_vec(),
            TagData::Short(short) => i16::to_le_bytes(*short).to_vec(),
            TagData::Int(int) => i32::to_le_bytes(*int).to_vec(),
            TagData::Float(float) => f32::to_le_bytes(*float).to_vec(),
            TagData::Double(double) => f64::to_le_bytes(*double).to_vec(),
            TagData::String(string) => {
                let mut bytes = Vec::<u8>::new();
                bytes.extend(u16::to_le_bytes(string.len() as u16));
                bytes.extend(string.as_bytes().to_vec());
                bytes
            }
            TagData::List(tag_type, size, list) => {
                let mut bytes = Vec::<u8>::new();
                bytes.push(tag_type.clone() as u8);
                bytes.extend(u32::to_le_bytes(*size));
                for i in 0..*size {
                    let data = &list[i as usize];
                    if data.kind() != *tag_type {
                        panic!("Type of tag does not match List tag");
                    }
                    bytes.extend(data.as_bytes());
                }
                bytes
            }
            TagData::Compound(compound) => {
                let mut bytes = Vec::<u8>::new();
                for tag in compound {
                    bytes.extend(tag.as_bytes());
                }
                bytes.push(0);
                bytes
            }
            _ => panic!("Unknown tag"),
        }
    }
    pub fn from_bytes(bytes: &[u8], ptr: &mut usize, kind: &TagKind) -> TagData {
        match kind {
            TagKind::Byte => TagData::Byte(i8::from_le_bytes([next_byte(bytes, ptr)])),
            TagKind::Short => TagData::Short(i16::from_le_bytes([
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
            ])),
            TagKind::Int => TagData::Int(i32::from_le_bytes([
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
            ])),
            TagKind::Long => TagData::Long(i64::from_le_bytes([
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
            ])),
            TagKind::Float => TagData::Float(f32::from_le_bytes([
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
            ])),
            TagKind::Double => TagData::Double(f64::from_le_bytes([
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
                next_byte(bytes, ptr),
            ])),
            TagKind::String => {
                let size = u16::from_le_bytes([next_byte(bytes, ptr), next_byte(bytes, ptr)]);

                let mut data: Vec<u8> = vec![];

                for _i in 0..size {
                    data.push(next_byte(bytes, ptr));
                }

                let string = String::from_utf8(data);
                match string {
                    Err(_error) => panic!("Invalid string"),
                    Ok(string) => TagData::String(string),
                }
            }
            TagKind::List => {
                let tag = TagKind::from_u8(next_byte(bytes, ptr));
                let size = u32::from_le_bytes([
                    next_byte(bytes, ptr),
                    next_byte(bytes, ptr),
                    next_byte(bytes, ptr),
                    next_byte(bytes, ptr),
                ]);

                let mut data = Vec::<TagData>::new();

                for _i in 0..size {
                    data.push(TagData::from_bytes(bytes, ptr, &tag))
                };

                TagData::List(tag, size, data)
            }
            TagKind::Compound => {
                let mut data = Vec::<NbtTag>::new();
                loop {
                    let kind = TagKind::from_u8(next_byte(bytes, ptr));
                    if let TagKind::End = kind {
                        break TagData::Compound(data);
                    }
                    data.push(NbtTag::from_bytes(bytes, ptr, &kind))
                }
            }
            _ => {
                panic!("Unknown tag 0x{:02X}", kind.clone() as u8);
            }
        }
    }
}
