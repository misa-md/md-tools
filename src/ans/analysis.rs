use std::fs::File;
use std::io::Read;
use crate::ans::voronoy::voronoy_ans;
use crate::ans::minio_input::voronoy_ans_minio;
use crate::ans::libminio_rw;

pub struct BoxConfig {
    pub box_size: Vec<u64>,
}

// wrap function for calling voronoy_ans, select to read from
// minio or local file system based in input config.
pub fn voronoy_ans_wrapper(xyzfile: &str, output: &str, input_from_minio: bool, box_config: &BoxConfig, verbose: bool) {
    if input_from_minio {
        let (mut data, data_ptr) = voronoy_ans_minio(xyzfile);
        if data.len() != 0 {
            let on_data_loaded = |atoms_size: usize| {
                unsafe {
                    libminio_rw::ReleaseMinioFile(data_ptr);
                };
                println!("atom size is {}", atoms_size);
            };
            voronoy_ans(&mut data, output, on_data_loaded, box_config, verbose);
        }
    } else {
        let mut input = File::open(xyzfile).unwrap(); // fixme: check file existence
        fn on_file_data_read(atoms_size: usize) {}
        voronoy_ans(&mut input.by_ref(), output, on_file_data_read, box_config, verbose);
    }
}
