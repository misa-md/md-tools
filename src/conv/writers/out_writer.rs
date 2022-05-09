use crate::conv::binary_types;

pub trait WriteProgress {
    // If one atom is read from binary file, this function will be called.
    fn on_atom_read(&mut self, atom: &binary_types::TypeAtom) -> i32;
    // before reading a new frame (before reading atoms for next time step)
    fn before_frame(&mut self, frame: u32, output: &str);
    // after reading a new frame (after reading atoms for next time step)
    fn after_frame(&mut self);
    // called before writing
    fn on_start(&mut self, output: &str);
    // called after all finished
    fn done(&mut self);
}
