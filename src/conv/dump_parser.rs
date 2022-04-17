// convert md binary result to format of lammps dump file.

use std::fs::{OpenOptions, File};
use std::io::{Seek, SeekFrom, Write};
use crate::conv::{binary_types, out_writer};

const DUMP_HEADER_MAX_SIZE: usize = 256; // max header size in bytes

pub struct DumpParser {
    output: std::io::BufWriter<File>,
    prec: usize,
    header_pos: u64,
    atom_count: u64,
    bound_min: (f64, f64, f64),
    bound_max: (f64, f64, f64),
}

impl DumpParser {
    // write header, include atom time step, number, bounds and atoms position.
    fn write_header(&mut self) {
        self.output.seek(SeekFrom::Start(self.header_pos)).unwrap();

        let fmt_string = format!("ITEM: TIMESTEP\n\
        {}\n\
        ITEM: NUMBER OF ATOMS\n\
        {}\n\
        ITEM: BOX BOUNDS pp pp pp\n\
        {} {} {}\n{} {} {}", 0, self.atom_count,
                                 self.bound_min.0 - 1e-4, self.bound_min.1 - 1e-4, self.bound_min.2 - 1e-4,
                                 self.bound_max.0 + 1e-4, self.bound_max.0 + 1e-4, self.bound_max.0 + 1e-4);
        let written_size = self.output.write(fmt_string.as_bytes()).unwrap(); // size of data of atom count

        if written_size >= DUMP_HEADER_MAX_SIZE {
            panic!("atoms count too big");
        } else {
            // fill the gap.
            let left_size = DUMP_HEADER_MAX_SIZE - written_size;
            let mut buf: Vec<u8> = vec![' ' as u8; left_size];
            buf[left_size - 1] = '\n' as u8;
            self.output.write(buf.as_slice()).unwrap();
        }
    }
    fn position(&mut self) -> u64 {
        // todo: use self.output.stream_position().unwrap() when the api is stable.
        self.output.seek(SeekFrom::Current(0)).unwrap()
    }
}

// In current implementation, we write atoms data into the result file first.
// Then, it will seek and write the header.
impl out_writer::WriteProgress for DumpParser {
    fn on_atom_read(&mut self, atom: &binary_types::TypeAtom) -> i32 {
        let fmt_string = format!("{} {} \t{:.*} \t{:.*} \t{:.*}\n",
                                 atom.id, atom.tp,
                                 self.prec, atom.atom_location[0],
                                 self.prec, atom.atom_location[1],
                                 self.prec, atom.atom_location[2]);
        self.output.write(fmt_string.as_bytes()).unwrap();
        // re-calculate bound
        if atom.atom_location[0] < self.bound_min.0 {
            self.bound_min.0 = atom.atom_location[0];
        }
        if atom.atom_location[1] < self.bound_min.1 {
            self.bound_min.1 = atom.atom_location[1];
        }
        if atom.atom_location[2] < self.bound_min.2 {
            self.bound_min.2 = atom.atom_location[2];
        }
        if atom.atom_location[0] > self.bound_max.0 {
            self.bound_max.0 = atom.atom_location[0];
        }
        if atom.atom_location[1] > self.bound_max.1 {
            self.bound_max.1 = atom.atom_location[1];
        }
        if atom.atom_location[2] > self.bound_max.2 {
            self.bound_max.2 = atom.atom_location[2];
        }

        self.atom_count += 1;
        return 1 as i32;
    }

    fn before_frame(&mut self, _frame: u32, _output_file: &str) {
        self.atom_count = 0;
        self.header_pos = self.position();
        self.output.seek(SeekFrom::Current(DUMP_HEADER_MAX_SIZE as i64)).unwrap();
        self.output.write("ITEM: ATOMS id type x y z\n".as_bytes()).unwrap();
    }

    fn after_frame(&mut self) {
        let bytes_to_end_frame: u64 = self.position();
        self.write_header();
        //seek back for processing out file with multiple frames.
        self.output.seek(SeekFrom::Start(bytes_to_end_frame));
    }

    fn on_start(&mut self, _output: &str) {
        // append to end of file.
        // we dont use append mode to write, because in this mode,
        // it always reposition cursor to end of file before each write.
        // see also, https://stackoverflow.com/q/10631862/10068476.
        self.output.seek(SeekFrom::End(0)).unwrap();
    }

    fn done(&mut self) {}
}

pub fn new_parser(filename: &str, precision: u32) -> DumpParser {
    // open output  file for writing.
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .truncate(false)
        .open(filename);

    match file {
        Ok(stream) => {
            return DumpParser {
                output: std::io::BufWriter::with_capacity(1024 * 1024, stream),
                prec: precision as usize,
                header_pos: 0,
                atom_count: 0,
                bound_min: (f64::MAX, f64::MAX, f64::MAX),
                bound_max: (f64::MIN, f64::MIN, f64::MIN),
            };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
