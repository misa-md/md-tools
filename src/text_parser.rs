use crate::ffi::ParseProgress;
use crate::ffi::OneAtomType;
use std::fs::{OpenOptions, File};
use std::io::Write;
use crate::ffi;

pub struct TextParser {
    output: File,
}

extern fn callback(target: *mut libc::c_void, atom: OneAtomType) -> libc::c_int { // this func is called by C.
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
        // todo
        return 1 as i32;
    }

    fn before_parsing(&self, output: &str) {}

    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, OneAtomType) -> libc::c_int {
        return callback;
    }

    fn finish_parsing(&self) {
        // close file.
    }
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
