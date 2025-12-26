macro_rules! define_conversion_from_fn {
    ($fn_name:ident, $type:ty, $size:literal) => {
        pub fn $fn_name(&mut self) -> $type {
            let bytes = self.next_bytes::<$size>();
            if self.little_endian {
                <$type>::from_le_bytes(bytes)
            } else {
                <$type>::from_be_bytes(bytes)
            }
        }
    };
}

macro_rules! define_conversion_to_fn {
    ($fn_name:ident, $type:ty, $size:literal) => {
        pub fn $fn_name(&mut self, num: $type) {
            if self.little_endian {
                self.bytes.extend(<$type>::to_le_bytes(num))
            } else {
                self.bytes.extend(<$type>::to_be_bytes(num))
            }
        }
    };
}

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

pub struct NbtReader {
    little_endian: bool,
    bytes: Vec<u8>,
    ptr: usize,
}

impl NbtReader {
    pub fn read_all(&self) -> bool {
        self.ptr >= self.bytes.len()
    }
    pub fn new(little_endian: bool, bytes: Vec<u8>) -> NbtReader {
        NbtReader { little_endian, bytes, ptr: 0 }
    }
    define_conversion_from_fn!{u16_from_next_bytes, u16, 2}
    define_conversion_from_fn!{u32_from_next_bytes, u32, 4}
    define_conversion_from_fn!{i16_from_next_bytes, i16, 2}
    define_conversion_from_fn!{i32_from_next_bytes, i32, 4}
    define_conversion_from_fn!{i64_from_next_bytes, i64, 8}
    define_conversion_from_fn!{f32_from_next_bytes, f32, 4}
    define_conversion_from_fn!{f64_from_next_bytes, f64, 8}
    pub fn next_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ptr];
        self.ptr += 1;
        byte
    }
    pub fn next_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut bytes = [0u8; N];
        for i in 0..N {
            bytes[i] = self.next_byte();
        }
        bytes
    }
}

pub struct NbtWriter {
    little_endian: bool,
    bytes: Vec<u8>,
}

impl NbtWriter {
    pub fn new(little_endian: bool) -> NbtWriter {
        NbtWriter { little_endian, bytes: vec![] }
    }
    define_conversion_to_fn!{u16_write_to_bytes, u16, 2}
    define_conversion_to_fn!{u32_write_to_bytes, u32, 4}
    define_conversion_to_fn!{i16_write_to_bytes, i16, 2}
    define_conversion_to_fn!{i32_write_to_bytes, i32, 4}
    define_conversion_to_fn!{i64_write_to_bytes, i64, 8}
    define_conversion_to_fn!{f32_write_to_bytes, f32, 4}
    define_conversion_to_fn!{f64_write_to_bytes, f64, 8}

    fn write(&mut self, byte: u8) {
        self.bytes.push(byte)
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
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
    pub fn as_bytes(&self, little_endian: bool) -> Vec<u8> {
        let mut w = NbtWriter::new(little_endian);

        for tag in &self.entries {
            tag.as_bytes(&mut w);
        }
        w.bytes
    }
    pub fn from_bytes(little_endian: bool, bytes: Vec<u8>) -> NbtTree {
        let mut r = NbtReader::new(little_endian, bytes);
        let mut tree = NbtTree::new(vec![]);
        loop {
            let kind = TagKind::from_u8(r.next_byte());
            tree.entries.push(NbtTag::from_bytes(&mut r, &kind));
            if r.read_all() {
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
    pub fn as_bytes(&self, w: &mut NbtWriter) {
        w.write(self.data.kind() as u8);
        w.u16_write_to_bytes(self.id.len() as u16);
        w.write_bytes(self.id.as_bytes());
        self.data.as_bytes(w);
    }
    pub fn from_bytes(r: &mut NbtReader, kind: &TagKind) -> NbtTag {
        let id_size = r.u16_from_next_bytes();

        let mut id = Vec::<u8>::new();
        for _i in 0..id_size {
            id.push(r.next_byte());
        }
        let id = String::from_utf8(id);
        match id {
            Err(_err) => panic!("Invalid id name"),
            Ok(id) => {
                return NbtTag {
                    id,
                    data: TagData::from_bytes(r, &kind),
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
            TagData::ByteArray(_size, list) => {
                if list.len() == 0 {
                    println!("\x1b[36mTAG_ByteArray\x1b[0m = []");
                    return;
                }
                println!("\x1b[36mTAG_ByteArray\x1b[0m = [");
                for data in list {
                    print!("{}", "  ".repeat(indent + 1));
                    println!("\x1b[33m{}\x1b[0m", data)
                }
                println!("{}]", "  ".repeat(indent));
            }
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
            TagData::IntArray(_size, list) => {
                if list.len() == 0 {
                    println!("\x1b[36mTAG_IntArray\x1b[0m = []");
                    return;
                }
                println!("\x1b[36mTAG_IntArray\x1b[0m = [");
                for data in list {
                    print!("{}", "  ".repeat(indent + 1));
                    println!("\x1b[33m{}\x1b[0m", data)
                }
                println!("{}]", "  ".repeat(indent));
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
    pub fn as_bytes(&self, w: &mut NbtWriter) {
        match self {
            TagData::Byte(byte) => w.write_bytes(&i8::to_be_bytes(*byte)),
            TagData::Short(short) => w.i16_write_to_bytes(*short),
            TagData::Int(int) => w.i32_write_to_bytes(*int),
            TagData::Float(float) => w.f32_write_to_bytes(*float),
            TagData::Double(double) => w.f64_write_to_bytes(*double),
            TagData::String(string) => {
                w.u16_write_to_bytes(string.len() as u16);
                w.write_bytes(string.as_bytes());
            }
            TagData::List(tag_type, size, list) => {
                w.write(tag_type.clone() as u8);
                w.u32_write_to_bytes(*size);
                for i in 0..*size {
                    let data = &list[i as usize];
                    if data.kind() != *tag_type {
                        panic!("Type of tag does not match List tag");
                    }
                    data.as_bytes(w);
                }
            }
            TagData::Compound(compound) => {
                for tag in compound {
                    tag.as_bytes(w);
                }
                w.write(0);
            }
            _ => panic!("Unknown tag"),
        }
    }
    pub fn from_bytes(r: &mut NbtReader, kind: &TagKind) -> TagData {
        match kind {
            TagKind::Byte => TagData::Byte(i8::from_le_bytes([r.next_byte()])),
            TagKind::Short => TagData::Short(r.i16_from_next_bytes()),
            TagKind::Int => TagData::Int(r.i32_from_next_bytes()),
            TagKind::Long => TagData::Long(r.i64_from_next_bytes()),
            TagKind::Float => TagData::Float(r.f32_from_next_bytes()),
            TagKind::Double => TagData::Double(r.f64_from_next_bytes()),
            TagKind::ByteArray => {
                let size = r.i32_from_next_bytes();

                let mut data = Vec::<i8>::new();

                for _i in 0..size {
                    data.push(i8::from_le_bytes([r.next_byte()]));
                }

                TagData::ByteArray(size, data)
            }
            TagKind::String => {
                let size = r.u16_from_next_bytes();

                let mut data: Vec<u8> = vec![];

                for _i in 0..size {
                    data.push(r.next_byte());
                }

                let string = String::from_utf8(data);
                match string {
                    Err(_error) => panic!("Invalid string"),
                    Ok(string) => TagData::String(string),
                }
            }
            TagKind::List => {
                let tag = TagKind::from_u8(r.next_byte());
                let size = r.u32_from_next_bytes();

                let mut data = Vec::<TagData>::new();

                for _i in 0..size {
                    data.push(TagData::from_bytes(r, &tag))
                };

                TagData::List(tag, size, data)
            }
            TagKind::Compound => {
                let mut data = Vec::<NbtTag>::new();
                loop {
                    let kind = TagKind::from_u8(r.next_byte());
                    if let TagKind::End = kind {
                        break TagData::Compound(data);
                    }
                    data.push(NbtTag::from_bytes(r, &kind))
                }
            }
            TagKind::IntArray => {
                let size = r.i32_from_next_bytes();

                let mut data = Vec::<i32>::new();

                for _i in 0..size {
                    data.push(r.i32_from_next_bytes())
                };

                TagData::IntArray(size, data)
            }
            _ => {
                panic!("Unknown tag 0x{:02X}", kind.clone() as u8);
            }
        }
    }
}
