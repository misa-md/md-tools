use std::fs::{OpenOptions, File};
use std::io::Write;
use crate::conv::binary_types::TypeAtom;
use crate::conv::writers::out_writer;

pub struct TextOutWriter {
    output: std::io::BufWriter<File>,
    prec: usize,
}

/**
// We create a buffered writer from the file we get
let mut writer = BufWriter::new(&file);
// Then we write to the file. write_all() calls flush() after the write as well.
writer.write_all(b"test\n");
 */
impl out_writer::WriteProgress for TextOutWriter {
    fn on_atom_read(&mut self, atom: &TypeAtom) -> i32 {
        let fmt_string = format!("{} \t {} \t{} \t{:.*} \t{:.*} \t{:.*} \t{:.*} \t{:.*} \t{:.*}\t{:.*} \t{:.*} \t{:.*}\n",
                                 atom.id, atom.get_name_by_ele_name(), atom.inter_type,
                                 self.prec, atom.atom_location[0],
                                 self.prec, atom.atom_location[1],
                                 self.prec, atom.atom_location[2],
                                 self.prec, atom.atom_velocity[0],
                                 self.prec, atom.atom_velocity[1],
                                 self.prec, atom.atom_velocity[2],
                                 self.prec, atom.atom_force[0],
                                 self.prec, atom.atom_force[1],
                                 self.prec, atom.atom_force[2]);
        self.output.write(fmt_string.as_bytes()).unwrap();
        return 1 as i32;
    }

    fn before_frame(&mut self, _frame: u32, _output: &str) {}

    fn after_frame(&mut self) {}

    //todo return Result<>
    fn on_start(&mut self, _output: &str) {
        // write header.
        self.output.write(b"id \tstep \ttype \tinter_type \tlocate.x \tlocate.y \tlocate.z \tv.x \tv.y \tv.z\n").unwrap();
    }

    //todo return Result<>
    fn done(&mut self) {}
}

// filename: output file.
pub fn new_writer(filename: &str, precision: u32) -> TextOutWriter {
    // open output  file for writing.
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(filename);

    match file {
        Ok(stream) => {
            return TextOutWriter {
                output: std::io::BufWriter::with_capacity(1024 * 1024, stream),
                prec: precision as usize,
            };
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
