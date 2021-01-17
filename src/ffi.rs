extern crate libc;

use std::ffi::CString;
use std::{fmt, error};

#[repr(C)]
pub struct OneAtomType {
    // u64
    pub  AtomId: libc::c_ulong,
    // fixme matching size_t in C side.
    pub  Step: libc::c_ulong,
    pub  AtomType: libc::c_int,
    pub  InterType: libc::c_short,
    // double 64
    pub  Location: [libc::c_double; 3],
    pub  Velocity: [libc::c_double; 3],
}

impl OneAtomType {
    pub fn get_name_by_ele_name(&self) -> &'static str {
        match self.AtomType {
            -1 => "V",
            0 => "Fe",
            1 => "Cu",
            2 => "Ni",
            _ => "Unknown",
        }
    }
}

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
    fn on_atom_read(&mut self, atom: &OneAtomType) -> i32;
    // todo return error
    fn before_parsing(&mut self, output: &str);
    //    fn parse(&mut self, filename: *const libc::c_char, ranks: libc::c_uint) -> libc::c_int;
    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, OneAtomType) -> libc::c_int;
    fn finish_parsing(&mut self);
}


extern {
    // passing filename, ranks and callback func (c will call rust fn as callback func).
    pub fn ParseBinaryAtoms(filename: *const libc::c_char, ranks: libc::c_uint, target: *mut libc::c_void,
                            on_atom_read: extern fn(*mut libc::c_void, OneAtomType) -> libc::c_int) -> libc::c_int;
}

//on_read: fn (atom: OneAtomType) -> u32
pub fn parse(filename: &str, output: &str, ranks: u32, mut progress: impl ParseProgress)
             -> std::result::Result<i32, ParseError> {
//    closure.Lock();
//    defer closure.Unlock();
    let obj = &progress as *const _ as *mut libc::c_void;

    progress.before_parsing(output);
    let filename_cstring = CString::new(filename).unwrap();
    let bytes = filename_cstring.as_bytes_with_nul();
    let status = unsafe {
        ParseBinaryAtoms(bytes.as_ptr() as *const libc::c_char,
                         ranks, obj, progress.load_callback())
    };
    if status != 0 {
        progress.finish_parsing();
        return Err(ParseError);
    } else {
        progress.finish_parsing();
        return Ok(1);
    }
}
