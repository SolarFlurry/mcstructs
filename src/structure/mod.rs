use core::panic;

use crate::{
    nbt::{NbtTree, TagData, TagKind, TagList},
    types::{Block, BlockState, BlockType, Vec3},
};

pub struct MCStructure {
    size: Vec3<i32>,
    blocks: Vec<i32>,
    palette: Vec<BlockType>,
    pub(in crate) block_position_data: Vec<(u32, TagData)>
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
            block_position_data: vec![]
        }
    }
    pub fn setblock(&mut self, loc: Vec3<i32>, block: BlockType) -> Block<'_> {
        if loc.x() >= self.size.x() || loc.y() >= self.size.y() || loc.z() >= self.size.z() {
            panic!("Location specified is out of structure bounds");
        }
        let index = self.size.z() * self.size.y() * loc.x() + self.size.z() * loc.y() + loc.z();
        self.blocks[index as usize] = self.palette.len() as i32;
        self.palette.push(block.clone());
        Block::new(block, index as u32, self)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut compound = TagData::Compound(TagList::new());

        // format_version
        compound.add_tag("format_version", TagData::Int(1));

        // size
        compound.add_tag(
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
        compound.add_tag(
            "structure",
            TagData::Compound(TagList::from(vec![
                (
                    "block_indices".to_string(),
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
                    )
                ),
                ("entities".to_string(), TagData::List(TagKind::Compound, 0, vec![])),
                (
                    "palette".to_string(),
                    TagData::Compound(TagList::from(vec![(
                        "default".to_string(),
                        TagData::Compound(TagList::from(vec![
                            (
                                "block_palette".to_string(),
                                TagData::List(
                                    TagKind::Compound,
                                    self.palette.len() as u32,
                                    self.palette
                                        .iter()
                                        .map(|value| {
                                            TagData::Compound(TagList::from(vec![
                                                (
                                                    "name".to_string(),
                                                    TagData::String(value.type_id.clone()),
                                                ),
                                                (
                                                    "states".to_string(),
                                                    TagData::Compound(TagList::from(
                                                        value
                                                            .states
                                                            .iter()
                                                            .map(|state| (
                                                                state.0.clone(),
                                                                match &state.1 {
                                                                    BlockState::String(string) => {
                                                                        TagData::String(
                                                                            string.to_string(),
                                                                        )
                                                                    }
                                                                    BlockState::Int(int) => {
                                                                        TagData::Int(*int)
                                                                    }
                                                                    BlockState::Bool(b) => {
                                                                        TagData::Byte(*b as i8)
                                                                    }
                                                                },
                                                            ))
                                                            .collect(),
                                                    )),
                                                ),
                                            ]))
                                        })
                                        .collect(),
                                ),
                            ),
                            (
                                "block_position_data".to_string(),
                                TagData::Compound(TagList::from(self.block_position_data.iter().map(|value| {
                                    println!("{}", value.0);
                                    (
                                    value.0.to_string(),
                                    TagData::Compound(TagList::from(vec![(
                                            "block_entity_data".to_string(),
                                            value.1.clone()
                                    )]))
                                )}).collect())),
                            ),
                        ])),
                    )])),
                ),
            ])),
        );

        // structure_world_origin
        compound.add_tag(
            "structure_world_origin",
            TagData::List(TagKind::Int, 3, vec![TagData::Int(0); 3]),
        );

        let nbt = NbtTree::new(vec![("".to_string(), compound)]);

        nbt.as_bytes(true)
    }
}
