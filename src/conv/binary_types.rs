/**
 * basic atom type saved in binary atom file.
 * each atom in binary atom file will include those information below.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TypeAtom {
    // atom id
    pub id: u64,
    // atom type
    pub tp: i32,
    pub inter_type: i16,
    // atom location
    pub atom_location: [f64; 3],
    // atom velocity
    pub atom_velocity: [f64; 3],
    // atom force
    pub atom_force: [f64; 3],
}

impl TypeAtom {
    pub fn get_name_by_ele_name(&self) -> &'static str {
        match self.tp {
            -1 => "V",
            0 => "Fe",
            1 => "Cu",
            2 => "Ni",
            _ => "Unknown",
        }
    }
}

pub trait BinaryParser {
    // return get total frames in global header
    fn global_header(&self) -> u32;
    // move next atom
    fn next(&mut self) -> bool;
    // decode atom struct in current position
    fn decode(&mut self) -> TypeAtom;
    // move to next frame
    fn move_to_next_frame(&mut self) -> bool;
    // get frame header
    fn frame_header(&self);
    // close parser
    fn close(&self);
}
