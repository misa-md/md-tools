/**
 * basic atom type saved in binary atom file.
 * each atom in binary atom file will include those information below.
 */
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct TypeAtom {
    // atom id
    id: u64,
    // atom type
    tp: i32,
    inter_type: i16,
    // atom location
    atom_location: [f64; 3],
    // atom velocity
    atom_velocity: [f64; 3],
    // atom force
    atom_force: [f64; 3],
}

trait BinaryParser {
    // get global header
    fn global_header(&self);
    // move next atom
    fn next(&self) -> bool;
    // decode atom struct in current position
    fn decode(&self) -> TypeAtom;
    // move to next frame
    fn move_to_next_frame(&self);
    // get frame header
    fn frame_header(&self);
}
