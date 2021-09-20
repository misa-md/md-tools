#[cfg(feature = "minio-analysis")]
use crate::ans::libminio_rw;
use std::ffi::{CStr, CString};
use std::ptr::{null_mut, null};

#[cfg(not(feature = "minio-analysis"))]
pub fn voronoy_ans_minio(_xyz_file: &str) -> (bool, &[u8], *mut libc::c_char) {
    println!("The feature of reading files from minio sever for analysis is disabled");
    // let data = String::from("Hello, world!").as_bytes();
    return (false, _xyz_file.as_bytes(), null_mut());
}

#[cfg(not(feature = "minio-analysis"))]
pub fn release_minio_file(_data_ptr: *mut libc::c_char) {
    return;
}

#[cfg(feature = "minio-analysis")]
pub fn release_minio_file(data_ptr: *mut libc::c_char) {
    unsafe {
        libminio_rw::ReleaseMinioFile(data_ptr);
    }
}

#[cfg(feature = "minio-analysis")]
pub fn voronoy_ans_minio(xyzfile: &str) -> (bool, &[u8], *mut libc::c_char) {
    let file = CString::new(xyzfile).unwrap();
    let status: libminio_rw::ReadMinioFile_return = unsafe {
        libminio_rw::ReadMinioFile(file.as_ptr() as *mut libc::c_char)
    };
    // status.r0 is data, status.r1 is size, status.r2 is error
    let err_str: &str = unsafe { CStr::from_ptr(status.r2) }.to_str().unwrap();
    if err_str != "" {
        println!("error: {}", err_str);
        return (false, &[], status.r0);
    }

    let data_size: u64 = status.r1 as u64; // libc::c_ulong (in fact, it is just u64) to u64
    let data = unsafe {
        std::ffi::CStr::from_ptr(status.r0)
    }.to_bytes();

    assert_eq!(data.len() as u64, data_size);
    println!("data size: {}", data_size);
    return (data.len() != 0, data, status.r0);
}
