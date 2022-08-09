use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Input {
    pub inp: u8,
}
