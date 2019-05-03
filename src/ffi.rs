extern crate libc;

use std::ptr;
use std::ffi::CString;
use std::{fmt, error};

#[repr(C)]
pub struct OneAtomType {
    // u64
    AtomId: libc::c_ulong,
    Step: libc::c_uint,
    AtomType: libc::c_int,
    InterType: libc::c_char,
    // double 64
    Location: [libc::c_double; 3],
    Velocity: [libc::c_double; 3],
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

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub trait ParseProgress {
    fn on_atom_read(&mut self, atom: &OneAtomType) -> i32;
    // todo return error
    fn before_parsing(&self, output: &str);
    //    fn parse(&mut self, filename: *const libc::c_char, ranks: libc::c_uint) -> libc::c_int;
    fn load_callback(&mut self) -> extern fn(*mut libc::c_void, OneAtomType) -> libc::c_int;
    fn finish_parsing(&self);
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
    let filename_c = CString::new(filename).unwrap();
    let status = unsafe {
        ParseBinaryAtoms(filename.as_ptr() as *const libc::c_char,
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
