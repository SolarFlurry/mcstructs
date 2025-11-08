use core::panic;
use serde::{Serialize, Deserialize};

use crate::{
    nbt::{NbtTag, NbtTree, TagData, TagKind},
    types::{BlockType, Vec3},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct MCStructure {
    size: Vec3<i32>,
    blocks: Vec<i32>,
    palette: Vec<BlockType>,
}

impl MCStructure {
    pub fn new(size: Vec3<i32>) -> MCStructure {
        let mut blocks: Vec<i32> = vec![];
        for _i in 0..(size.x() * size.y() * size.z()) {
            blocks.push(-1);
        }
        MCStructure {
            size,
            blocks,
            palette: vec![],
        }
    }
    pub fn setblock(&mut self, loc: Vec3<i32>, block: BlockType) {
        if loc.x() >= self.size.x() || loc.y() >= self.size.y() || loc.z() >= self.size.z() {
            panic!("Location specified is out of structure bounds");
        }
        let index = self.size.z() * self.size.y() * loc.x() + self.size.z() * loc.y() + loc.z();
        self.blocks[index as usize] = self.palette.len() as i32;
        self.palette.push(block);
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut compound = NbtTag {
            id: "".to_string(),
            data: TagData::Compound(vec![]),
        };

        // format_version
        compound.data.add_tag("format_version", TagData::Int(1));

        // size
        compound.data.add_tag(
            "size",
            TagData::List(
                TagKind::Int,
                3,
                vec![
                    TagData::Int(*self.size.x()),
                    TagData::Int(*self.size.y()),
                    TagData::Int(*self.size.z()),
                ],
            ),
        );

        // block_indices
        compound.data.add_tag(
            "block_indices",
            TagData::List(
                TagKind::List,
                2,
                vec![
                    TagData::List(
                        TagKind::Int,
                        self.blocks.len() as u32,
                        self.blocks
                            .iter()
                            .map(|value| TagData::Int(*value))
                            .collect(),
                    ),
                    TagData::List(
                        TagKind::Int,
                        self.blocks.len() as u32,
                        vec![TagData::Int(-1); self.blocks.len()],
                    ),
                ],
            ),
        );

		// entities
		compound.data.add_tag("entities", TagData::List(TagKind::Compound, 0, vec![]));

        // palette
        compound.data.add_tag(
            "palette",
            TagData::Compound(vec![NbtTag {
                id: "default".to_string(),
                data: TagData::Compound(vec![NbtTag {
                    id: "block_palette".to_string(),
                    data: TagData::List(
                        TagKind::Compound,
                        self.palette.len() as u32,
                        self.palette.iter().map(|value| TagData::Compound(vec![
							NbtTag {id: "name".to_string(), data: TagData::String(value.namespace.clone())}
						])).collect(),
                    ),
                }, NbtTag {id: "block_position_data".to_string(), data: TagData::Compound(vec![])}]),
            }]),
        );

        // structure_world_origin
		compound.data.add_tag("structure_world_origin", TagData::List(TagKind::Int, 3, vec![
			TagData::Int(0); 3
		]));

        let nbt = NbtTree::new(vec![compound]);

        nbt.as_bytes()
    }
}
