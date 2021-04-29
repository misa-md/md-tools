use std::fs::{OpenOptions, File};
use std::io::Write;
use crate::conv::binary_parser::{ParseProgress};
use crate::conv::binary_types::TypeAtom;

pub struct TextParser {
    output: File,
}

extern fn text_callback(target: *mut libc::c_void, atom: TypeAtom) -> libc::c_int { // this func is called by C.
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
    fn on_atom_read(&mut self, atom: &TypeAtom) -> i32 {
        let fmt_string = format!("{} \t {} \t{} \t{:.6} \t{:.6} \t{:.6}  \t{:.6} \t{:.6} \t{:.6}\n",
                                 atom.id, atom.get_name_by_ele_name(), atom.inter_type,
                                 atom.atom_location[0], atom.atom_location[1], atom.atom_location[2],
                                 atom.atom_velocity[0], atom.atom_velocity[1], atom.atom_velocity[2]);
        self.output.write(fmt_string.as_bytes()).unwrap();
        return 1 as i32;
    }

    //todo return Result<>
    fn before_parsing(&mut self, _output: &str) {
        // write header.
        self.output.write(b"id \tstep \ttype \tinter_type \tlocate.x \tlocate.y \tlocate.z \tv.x \tv.y \tv.z\n").unwrap();
    }

    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, TypeAtom) -> libc::c_int {
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
        Ok(stream) => {
            return TextParser { output: stream };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
