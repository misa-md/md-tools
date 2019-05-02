use crate::ffi::ParseProgress;
use crate::ffi::OneAtomType;
use std::fs::{OpenOptions, File};
use std::io::Write;

pub struct TextParser {
    output: File,
}

/**
// We create a buffered writer from the file we get
let mut writer = BufWriter::new(&file);
// Then we write to the file. write_all() calls flush() after the write as well.
writer.write_all(b"test\n");
*/

impl ParseProgress for TextParser {
    fn on_atom_read(&mut self, atom: &OneAtomType) {
        self.output.write(b"@{} u");
    }

    fn before_parsing(&self, output: &str) {}

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
