extern crate libc;

use std::{fmt, error};
use crate::conv::binary_parser_v1::make_parser;
use crate::conv::{binary_parser_v2};
use crate::conv::binary_types::{BinaryParser};
use crate::conv::out_writer::WriteProgress;


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

//on_read: fn (atom: OneAtomType) -> u32
// select parser for different version of binary format
pub fn parse_wrapper(bin_standard: &str, filename: &str, output: &str, ranks: u32, writer: impl WriteProgress)
                     -> std::result::Result<i32, ParseError> {
    if bin_standard == "current" {
        let bin_parser = match make_parser(filename, ranks) {
            Ok(p) => p,
            Err(e) => return Err(e)
        };
        return parse(output, bin_parser, writer);
    }

    if bin_standard == "next" {
        let bin_parser_v2 = match binary_parser_v2::make_parser(filename) {
            Ok(p) => p,
            Err(e) => return Err(e)
        };
        return parse(output, bin_parser_v2, writer);
    }

    return Ok(1);
}

fn parse(output: &str, mut parser: impl BinaryParser, mut writer: impl WriteProgress)
         -> std::result::Result<i32, ParseError> {
    let frames = parser.global_header();
    writer.on_start(output);
    for frame in 0..frames {
        parser.move_to_next_frame();
        writer.before_frame(frame, output);
        while parser.next() {
            let atom = parser.decode();
            if atom.tp != -1 { // invalid atom
                writer.on_atom_read(&atom);
            }
            // println!("{:?}", atom);
        }
        writer.after_frame();
    }
    writer.done();
    parser.close();

    return Ok(1);
}
