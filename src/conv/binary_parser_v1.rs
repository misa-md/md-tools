use std::ffi::CString;

use crate::conv::{binary_types, lib_conv_capi};
use crate::conv::binary_parser::ParseError;

pub struct BinaryParserV1 {
    c_parser: *mut ::std::os::raw::c_void,
    atom: lib_conv_capi::type_c_atom,
}

// initialize parser
pub fn make_parser(filename: &str, ranks: u32)
                   -> std::result::Result<BinaryParserV1, ParseError> {
    let filename_cstring = CString::new(filename).unwrap();
    let bytes = filename_cstring.as_bytes_with_nul();
    // it returns a parser pointer of C side
    let binary_parser = unsafe {
        lib_conv_capi::make_parse(bytes.as_ptr() as *const libc::c_char, ranks)
    };

    if binary_parser == 0 as *mut ::std::os::raw::c_void { // == NULL
        return Err(ParseError);
    } else {
        let binary_parser = BinaryParserV1 {
            c_parser: binary_parser,
            atom: lib_conv_capi::type_c_atom {
                id: 0,
                step: 0,
                type_: 0,
                inter_type: 0,
                atom_location: [0.0, 0.0, 0.0],
                atom_velocity: [0.0, 0.0, 0.0],
            },
        };
        return Ok(binary_parser);
    }
}


impl binary_types::BinaryParser for BinaryParserV1 {
    fn global_header(&self) -> u32 {
        // only support 1 frame
        return 1;
    }

    fn next(&mut self) -> bool {
        let ok = unsafe {
            lib_conv_capi::read_next_atom(self.c_parser, &mut self.atom as *mut lib_conv_capi::type_c_atom)
        };
        return ok == 0;
    }

    fn decode(&mut self) -> binary_types::TypeAtom {
        let atom = binary_types::TypeAtom {
            id: self.atom.id,
            tp: self.atom.type_,
            inter_type: 0,
            atom_location: self.atom.atom_location,
            atom_velocity: self.atom.atom_velocity,
            atom_force: [0.0, 0.0, 0.0],
        };
        return atom;
    }

    fn move_to_next_frame(&mut self) -> bool {
        return false;
    }

    fn frame_header(&self) {
    }

    fn close(&self) {
        unsafe {
            lib_conv_capi::close_parser(self.c_parser);
        };
    }
}
