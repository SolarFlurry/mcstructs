use crate::{structure::MCStructure, types::Vec3};

#[repr(C)]
pub struct C_MCStructure {
    structure: *mut MCStructure,
}

#[repr(C)]
pub struct C_Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn vec3(x: i32, y: i32, z: i32) -> C_Vec3 {
    C_Vec3 { x, y, z }
}

#[unsafe(no_mangle)]
pub extern "C" fn mcstructure_new(size: C_Vec3) -> C_MCStructure {
    C_MCStructure {
        structure: Box::into_raw(Box::new(MCStructure::new(Vec3::new(
            size.x, size.y, size.z,
        )))),
    }
}
