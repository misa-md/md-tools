use crate::ffi::ParseProgress;
use crate::ffi::OneAtomType;
use std::fs::{OpenOptions, File};
use std::io::Write;

pub struct TextParser {
    output: File,
}

extern fn text_callback(target: *mut libc::c_void, atom: OneAtomType) -> libc::c_int { // this func is called by C.
    let text_parser: &mut TextParser = unsafe { &mut *(target as *mut TextParser) };
    text_parser.on_atom_read(&atom) as libc::c_int
}

/**
// We create a buffered writer from the file we get
let mut writer = BufWriter::new(&file);
// Then we write to the file. write_all() calls flush() after the write as well.
writer.write_all(b"test\n");
*/
impl ParseProgress for TextParser {
    fn on_atom_read(&mut self, atom: &OneAtomType) -> i32 {
        let fmt_string = format!("{} \t{} \t{} \t{} \t{:.6} \t{:.6} \t{:.6}  \t{:.6} \t{:.6} \t{:.6}\n",
                                 atom.AtomId, atom.Step, atom.get_name_by_ele_name(), atom.InterType,
                                 atom.Location[0], atom.Location[1], atom.Location[2],
                                 atom.Velocity[0], atom.Velocity[1], atom.Velocity[2]);
        self.output.write(fmt_string.as_bytes());
        return 1 as i32;
    }

    //todo return Result<>
    fn before_parsing(&mut self, output: &str) {
        // write header.
        self.output.write(b"id \tstep \ttype \tinter_type \tlocate.x \tlocate.y \tlocate.z \tv.x \tv.y \tv.z\n");
    }

    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, OneAtomType) -> libc::c_int {
        return text_callback;
    }
    //todo return Result<>
    fn finish_parsing(&mut self) {}
}

// filename: output file.
pub fn new_parser(filename: &str) -> TextParser {
    // open output  file for writing.
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(filename);

    match file {
        Ok(mut stream) => {
            return TextParser { output: stream };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
