use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct NbtTree {
    entries: Vec<NbtTag>,
}

impl NbtTree {
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
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NbtTag {
    pub id: String,
    pub data: TagData,
}
impl NbtTag {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
		bytes.push(self.data.kind() as u8);
        bytes.extend(u16::to_le_bytes(self.id.len() as u16));
        bytes.extend_from_slice(self.id.as_bytes());
        bytes.extend(self.data.as_bytes());

        bytes
    }
}

#[derive(Clone, Serialize, Deserialize)]
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
			TagData::List(tag_type, size,  list) => {
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
}
