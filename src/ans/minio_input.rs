use crate::ans::libminio_rw;
use std::ffi::{CStr, CString};

pub fn voronoy_ans_minio(xyzfile: &str) -> (&[u8], *mut libc::c_char) {
    let file = CString::new(xyzfile).unwrap();
    let status: libminio_rw::ReadMinioFile_return = unsafe {
        libminio_rw::ReadMinioFile(file.as_ptr() as *mut libc::c_char)
    };
    // status.r0 is data, status.r1 is size, status.r2 is error
    let err_str: &str = unsafe { CStr::from_ptr(status.r2) }.to_str().unwrap();
    if err_str != "" {
        println!("error: {}", err_str);
        return (&[], status.r0);
    }

    let data_size: u64 = status.r1 as u64; // libc::c_ulong (in fact, it is just u64) to u64
    let data = unsafe {
        std::ffi::CStr::from_ptr(status.r0)
    }.to_bytes();

    assert_eq!(data.len() as u64, data_size);
    println!("data size: {}", data_size);
    return (data, status.r0);
}
