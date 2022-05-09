use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use serde::{Serialize, Deserialize};
use crate::conv::binary_types::TypeAtom;
use crate::conv::writers::out_writer;

// parse the origin binary file to another binary file.
pub struct BinOutWriter {
    output: std::io::BufWriter<File>,
    atom_count: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct BinaryAtom {
    id: u64,
    tp: i64, // padding i32 to i64
    pos: (f64, f64, f64),
    v: (f64, f64, f64),
    f: (f64, f64, f64),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Header {
    uniq: u64,
    atom_count: i64,
    version: u32,
    mask: u32,
    atom_size: usize,
}

impl out_writer::WriteProgress for BinOutWriter {
    fn on_atom_read(&mut self, atom: &TypeAtom) -> i32 {
        let entity = BinaryAtom {
            id: atom.id,
            tp: atom.tp as i64,
            pos: (atom.atom_location[0], atom.atom_location[1], atom.atom_location[2]),
            v: (atom.atom_velocity[0], atom.atom_velocity[1], atom.atom_velocity[2]),
            f: (atom.atom_force[0], atom.atom_force[1], atom.atom_force[2]),
        };
        let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();
        self.output.write(&*encoded).unwrap();
        self.atom_count = self.atom_count + 1;
        return 1 as i32;
    }

    fn before_frame(&mut self, _frame: u32, _output: &str) {}

    fn after_frame(&mut self) {
        // write file header
        let h = Header {
            uniq: 0x6869206870636572,
            atom_count: self.atom_count,
            version: 0x01,
            mask: 0x7,
            atom_size: std::mem::size_of::<BinaryAtom>()
        };
        let encoded: Vec<u8> = bincode::serialize(&h).unwrap();
        self.output.seek(SeekFrom::Start(0)).unwrap();
        self.output.write(&encoded).unwrap();
    }

    //todo return Result<>
    fn on_start(&mut self, _output: &str) {
        self.output.seek(SeekFrom::Start(std::mem::size_of::<Header>() as u64)).unwrap();
    }

    //todo return Result<>
    fn done(&mut self) {}
}

// filename: output file.
pub fn new_writer(filename: &str, _precision: u32) -> BinOutWriter {
    // open output  file for writing.
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(filename);

    match file {
        Ok(stream) => {
            return BinOutWriter {
                output: std::io::BufWriter::with_capacity(1024 * 1024, stream),
                atom_count: 0,
            };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
