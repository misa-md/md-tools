extern crate libc;

use std::{fmt, error};
use crate::conv::binary_parser_v1::make_parser;
use crate::conv::binary_types;
use crate::conv::binary_types::{BinaryParser};


#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error happened while processing binary atom file")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ParseError {
    fn description(&self) -> &str {
        "error happened while processing binary atom file"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub trait ParseProgress {
    fn on_atom_read(&mut self, atom: &binary_types::TypeAtom) -> i32;
    // todo return error
    fn before_parsing(&mut self, output: &str);
    //    fn parse(&mut self, filename: *const libc::c_char, ranks: libc::c_uint) -> libc::c_int;
    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, binary_types::TypeAtom) -> libc::c_int;
    fn finish_parsing(&mut self);
}

//on_read: fn (atom: OneAtomType) -> u32
pub fn parse(filename: &str, output: &str, ranks: u32, mut progress: impl ParseProgress)
             -> std::result::Result<i32, ParseError> {
    let mut bin_parser = match make_parser(filename, output, ranks) {
        Ok(p) => p,
        Err(e) => return Err(e)
    };
    progress.before_parsing(output);

    while bin_parser.next() {
        let atom = bin_parser.decode();
        progress.on_atom_read(&atom);
    }
    bin_parser.close();
    progress.finish_parsing();
    return Ok(1);
}
