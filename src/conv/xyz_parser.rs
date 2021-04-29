use std::io::{Write, SeekFrom};
use std::io::Seek;
use std::fs::{OpenOptions, File};
use crate::conv::{binary_types, out_writer};

pub struct XYZParser {
    output: File,
    atom_count: u64,
}

impl XYZParser {
    fn write_auto_header(&mut self) {
        // write header.
        self.output.seek(SeekFrom::Start(0)).unwrap();
        let fmt_string = format!("{}\n", self.atom_count);
        let written_size = self.output.write(fmt_string.as_bytes()).unwrap(); // size of data of atom count
        if written_size >= 128 {
            panic!("atoms count too big");
        } else {
            let left_size = 128 - written_size;
            let mut buf: Vec<u8> = vec!['C' as u8; left_size];
            buf[left_size - 1] = '\n' as u8;
            self.output.write(buf.as_slice()).unwrap();
        }
    }
}

impl out_writer::WriteProgress for XYZParser {
    fn on_atom_read(&mut self, atom: &binary_types::TypeAtom) -> i32 {
        let fmt_string = format!("{} \t{:.6} \t{:.6} \t{:.6}\n",
                                 atom.get_name_by_ele_name(),
                                 atom.atom_location[0], atom.atom_location[1], atom.atom_location[2]);
        self.output.write(fmt_string.as_bytes()).unwrap();
        self.atom_count += 1;
        return 1 as i32;
    }

    fn before_frame(&mut self, frame: u32, output: &str) {
        self.atom_count = 0;
        self.output.seek(SeekFrom::Start(128)).unwrap();
    }

    fn after_frame(&mut self) {
        self.write_auto_header();
    }

    fn on_start(&mut self, output: &str) {}

    fn done(&mut self) {}
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
        Ok(stream) => {
            return XYZParser { output: stream, atom_count: 0 };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
