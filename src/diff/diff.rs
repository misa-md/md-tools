use std::fs::File;
use crate::xyz::xyz_reader::{Reader, Snapshot};
use crate::xyz::particle::Particle;

pub fn diff_wrapper(file1: &str, file2: &str, error_limit: f64, periodic_checking: bool, box_measured_size: (f64, f64, f64)) {
    let input_1 = File::open(file1).unwrap();
    let input_2 = File::open(file2).unwrap();

    let mut reader1 = Reader::new(input_1);
    let mut reader2 = Reader::new(input_2);

    let snapshot_result_1 = reader1.read_snapshot();
    match snapshot_result_1 {
        Err(e) => {
            panic!("read input xyz file error: {:?}", e);
        }
        Ok(ref _snapshot) => {}
    }

    let snapshot_result_2 = reader2.read_snapshot();
    match snapshot_result_2 {
        Err(e) => {
            panic!("read input xyz file error: {:?}", e);
        }
        Ok(ref _snapshot) => {}
    }

    diff(&mut snapshot_result_1.unwrap(), &mut snapshot_result_2.unwrap(), error_limit, periodic_checking, box_measured_size);
}

fn diff(snapshot1: &mut Snapshot<Particle>, snapshot2: &mut Snapshot<Particle>, error_limit: f64, periodic_checking: bool, box_measured_size: (f64, f64, f64)) {
    if snapshot1.size() != snapshot2.size() {
        println!("mismatched atom size in two files");
        return;
    }
    let cmp = |a: &Particle, b: &Particle| {
        a.id.cmp(&b.id)
    };
    snapshot1.atoms.sort_by(cmp);
    snapshot2.atoms.sort_by(cmp);

    let mut same_flag = true;
    let num_atoms = snapshot1.size();
    if periodic_checking {
        for i in 0..num_atoms {
            if !(snapshot1.atoms[i].is_status_near_eq_with_pbc(&(snapshot2.atoms[i]), error_limit, box_measured_size)) {
                same_flag = false;
                println!("mismatch atom position or velocity: \n{}\n{}",
                         snapshot1.atoms[i].to_string(), snapshot2.atoms[i].to_string());
            }
        }
    } else {
        for i in 0..num_atoms {
            if !(snapshot1.atoms[i].is_status_near_eq(&(snapshot2.atoms[i]), error_limit)) {
                same_flag = false;
                println!("mismatch atom position or velocity: \n{}\n{}",
                         snapshot1.atoms[i].to_string(), snapshot2.atoms[i].to_string());
            }
        }
    }
    if same_flag {
        println!("no difference.")
    }
}
