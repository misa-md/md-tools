/**
 * Created by genshen at 2020/12/25
 */
use std::fs::File;
use std::io::Read;
use std::io;

use crate::ans::minio_input;
use crate::ans::box_config::{BoxConfig, config_simulation_box};
use crate::ans::voronoy::do_analysis_wrapper;
use crate::xyz::xyz_reader::Reader;


// wrap function for calling read_atoms_and_analysis,
// select to read from minio or local file system based in input config.
pub fn analysis_wrapper(xyzfile: &str, output: &str, input_from_minio: bool, box_config: &mut BoxConfig, verbose: bool) {
    if input_from_minio {
        let (ok, mut data, data_ptr) = minio_input::voronoy_ans_minio(xyzfile);
        if ok {
            let on_data_loaded = |atoms_size: usize| {
                minio_input::release_minio_file(data_ptr);
                println!("atom size is {}", atoms_size);
            };
            read_atoms_and_analysis(&mut data, output, on_data_loaded, box_config, verbose);
        }
    } else {
        let mut input = File::open(xyzfile).unwrap(); // fixme: check file existence
        fn on_file_data_read(_atoms_size: usize) {}
        read_atoms_and_analysis(&mut input.by_ref(), output, on_file_data_read, box_config, verbose);
    }
}

// voronoy analysis method for BCC lattice and cube lattice.
// todo: on data read error
pub fn read_atoms_and_analysis<R: ?Sized>(input: &mut R, output: &str, on_data_loaded: impl Fn(usize),
                                          box_config: &mut BoxConfig, verbose: bool)
    where R: io::Read
{
    let mut reader = Reader::new(input);
    // todo read atom one by one and compute its index lattice.
    let snapshot_result = reader.read_snapshot();

    match snapshot_result {
        Err(e) => {
            println!("read input xyz file error: {:?}", e);
        }
        Ok(snapshot) => {
            let atoms_size = snapshot.size();
            on_data_loaded(atoms_size);
            if config_simulation_box(&snapshot, box_config, verbose) {
                // do analysis
                do_analysis_wrapper(output, box_config.box_size_, box_config.box_start, &snapshot);
            } else {
                println!("config simulation box failed");
            }
        }
    }
}
