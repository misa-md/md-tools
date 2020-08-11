use crate::ans::libminio_rw;
use std::ffi::{CStr, CString};
use xyzio::Reader;

pub fn voronoy_ans_minio(xyzfile: &str, output: &str) {
    let file = CString::new(xyzfile).unwrap();
    let status: libminio_rw::ReadMinioFile_return = unsafe {
        libminio_rw::ReadMinioFile(file.as_ptr() as *mut libc::c_char)
    };
    // status.r0 is data, status.r1 is size, status.r2 is error
    let err_str: &str = unsafe { CStr::from_ptr(status.r2) }.to_str().unwrap();
    if err_str != "" {
        println!("error: {}", err_str);
    }

    let data_size: u64 = status.r1 as u64; // libc::c_ulong (in fact, it is just u64) to u64
    let data = unsafe {
        std::ffi::CStr::from_ptr(status.r0)
    }.to_bytes();

    println!("data size: {}", data_size);
    let mut reader = Reader::new(data);
    // todo read atom one by one and compute its index lattice.
    let snapshot_result = reader.read_snapshot();
    println!("atom size is {}", snapshot_result.unwrap().atoms.len());

    unsafe {
        libminio_rw::ReleaseMinioFile(status.r0);
    };
}
