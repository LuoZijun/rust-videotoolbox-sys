
use crate::libc::int32_t;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Point {
    pub x: int32_t,
    pub y: int32_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Size {
    pub width: int32_t,
    pub height: int32_t,
}
