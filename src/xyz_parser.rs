use std::io::{Write, SeekFrom};
use std::io::Seek;
use std::io;
use std::fs::{OpenOptions, File};
use crate::ffi::{ParseProgress, OneAtomType};
use crate::ffi;

pub struct XYZParser {
    output: File,
    atom_count: u64,
}

extern fn xyz_callback(target: *mut libc::c_void, atom: OneAtomType) -> libc::c_int { // this func is called by C.
    let xyz_parser: &mut XYZParser = unsafe { &mut *(target as *mut XYZParser) };
    xyz_parser.on_atom_read(&atom) as libc::c_int
}

impl ParseProgress for XYZParser {
    fn on_atom_read(&mut self, atom: &OneAtomType) -> i32 {
        let fmt_string = format!("{} \t{:.6} \t{:.6} \t{:.6}\n",
                                 atom.getNameByEleName(),
                                 atom.Location[0], atom.Location[1], atom.Location[2]);
        self.output.write(fmt_string.as_bytes());
        self.atom_count += 1;
        return 1 as i32;
    }

    fn before_parsing(&mut self, output: &str) {
        self.output.seek(SeekFrom::Start(128));
    }

    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, OneAtomType) -> libc::c_int {
        return xyz_callback;
    }

    fn finish_parsing(&mut self) {
        // write header.
        self.output.seek(SeekFrom::Start(0));
        let fmt_string = format!("{}\n", self.atom_count);
        let written_size = self.output.write(fmt_string.as_bytes()).unwrap(); // size of data of atom count
        if written_size >= 128 {
            panic!("atoms count too big");
        } else {
            let left_size = 128 - written_size;
            let mut buf: Vec<u8> = vec!['C' as u8; left_size];
            buf[left_size - 1] = '\n' as u8;
            self.output.write(buf.as_slice());
        }
    }
}

// filename: output file.
pub fn new_parser(filename: &str) -> XYZParser {
    // open output  file for writing.
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(filename);

    match file {
        Ok(mut stream) => {
            return XYZParser { output: stream, atom_count: 0 };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
