use crate::{
    structure::MCStructure as RustMcStructure,
    types::{Vec3 as RustVec3, BlockType as RustBlockType}
};

#[repr(C)]
pub struct StringSlice {
    ptr: *const u8,
    len: usize,
}

impl StringSlice {
    fn to_string(&self) -> &str {
        let slice = unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        };

        str::from_utf8(slice).unwrap()
    }
}

#[repr(C)]
pub struct McStructure {
    structure: *mut RustMcStructure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockType {
    inner: *mut RustBlockType,
}

#[repr(C)]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn to_other(&self) -> RustVec3<i32> {
        RustVec3::new(self.x, self.y, self.z)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3 { x, y, z }
}

#[unsafe(no_mangle)]
pub extern "C" fn mcstructure_new(size: Vec3) -> McStructure {
    McStructure {
        structure: Box::into_raw(Box::new(RustMcStructure::new(RustVec3::new(
            size.x, size.y, size.z,
        )))),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mcstructure_free(structure: *mut McStructure) {
    let _free_ptr = unsafe { Box::from_raw((*structure).structure) };
}

#[unsafe(no_mangle)]
pub extern "C" fn mcstructure_as_bytes(structure: *mut McStructure, ptr: *mut*mut u8, capacity: *mut usize) -> usize {
    let structure = unsafe { Box::from_raw((*structure).structure) };
    let mut bytes = structure.as_bytes();

    unsafe { *ptr = bytes.as_mut_ptr(); }
    unsafe { *capacity = bytes.capacity(); }

    let len = bytes.len();

    std::mem::forget(bytes);

    return len;
}

#[unsafe(no_mangle)]
pub extern "C" fn mcstructure_free_bytes(ptr: *mut u8, capacity: usize, len: usize) {
    let _ = unsafe { Vec::from_raw_parts(ptr, len, capacity) };
}

#[unsafe(no_mangle)]
pub extern "C" fn mcstructure_setblock(structure: *mut McStructure, loc: Vec3, block: BlockType) {
    let mut structure = unsafe { Box::from_raw((*structure).structure) };
    structure.setblock(loc.to_other(), *unsafe { Box::from_raw(block.inner) });
}

#[unsafe(no_mangle)]
pub extern "C" fn blocktype_new(namespace: StringSlice) -> BlockType {
    BlockType {
        inner: Box::into_raw(Box::new(RustBlockType::new(namespace.to_string()))),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn blocktype_free(block: *mut BlockType) {
    let _free_ptr = unsafe { Box::from_raw((*block).inner) };
}