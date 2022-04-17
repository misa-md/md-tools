use clap::Parser;

use std::path::{Path, PathBuf};
use crate::cli::AnsAlgorithm;

use crate::conv::{binary_parser, xyz_parser, text_parser, dump_parser};

mod ans;
mod diff;
mod xyz;
mod conv;
mod cli;

fn main() {
    let args = cli::Cli::parse();

    match &args.command {
        cli::Commands::Conv {
            dry, input, output, format,
            precision, standard, ranks
        } => {
            parse_convert(dry.clone(), standard.clone(), &input, output.clone(), format.clone(), ranks.clone(), precision.clone());
            return;
        }
        cli::Commands::Diff { error, file_1, file_2, periodic_checking, sim_box } => {
            parse_diff(error.clone(), file_1.clone(), file_2.clone(), periodic_checking.clone(), sim_box);
            return;
        }
        cli::Commands::Ans {
            input, output, verbose, input_from_minio, box_start,
            box_size, algorithm
        } => {
            parse_ans(input, output, verbose.clone(), input_from_minio.clone(), box_start.clone(), box_size.clone(), algorithm.clone());
            return;
        }
    }
}

fn parse_convert(dry_run: bool, bin_standard: cli::FormatStandard, input_files: &Vec<PathBuf>, output: String,
                 format: cli::OutFormat, ranks: usize, precision: u32) {
    if ranks <= (0 as usize) {
        println!("unsupported ranks value.");
        return;
    }
    if !(format == cli::OutFormat::Xyz || format == cli::OutFormat::Dump || format == cli::OutFormat::Text) {
        println!("unsupported format.");
        return;
    }

    if input_files.len() == 0 {
        println!("no matching input files");
        return;
    }
    let multiple_outputs = if input_files.len() != 1 && format != cli::OutFormat::Dump {
        true
    } else {
        false
    };

    for input_file in input_files {
        let input_path = Path::new(input_file);
        println!("converting file {}", input_file.to_str().unwrap());

        let output_file_path = if multiple_outputs {
            let output_prefix = input_path.file_name().unwrap().to_str().unwrap();
            format!("{}.{}", output_prefix, output)
        } else {
            output.to_string()
        };

        if !dry_run {
            mk_parse(format, precision, bin_standard, ranks as u32, input_file.to_str().unwrap(), output_file_path.as_str());
        }
        println!("file {} converted, saved at {}", input_file.to_str().unwrap(), output_file_path.as_str());
    }
}

fn parse_ans(input: &Vec<PathBuf>, output: &Vec<String>, verbose_log: bool, input_from_minio: bool,
             _box_start: Vec<f64>, box_size: Vec<u64>, _algorithm: AnsAlgorithm) {
    if box_size.len() != 0 && box_size.len() != 3 {
        panic!("length of box size must be 3.");
    }
    if _box_start.len() != 0 && _box_start.len() != 3 {
        panic!("length of box start must be 3.");
    }

    let input_files = input.clone();
    let output_files = output.clone();
    let mut box_start: Vec<ans::voronoy::Float> = Vec::new();
    for start in _box_start {
        box_start.push(start as ans::voronoy::Float);
    }

    let mut box_config = ans::box_config::BoxConfig {
        input_box_start: box_start,
        input_box_size: box_size,
        box_size_: (0, 0, 0),
        box_start: (0.0, 0.0, 0.0),
    };

    if input_files.len() == 0 {
        println!("no matching input files");
        return;
    }
    if output_files.len() == input_files.len() {
        for i in 0..output_files.len() {
            println!("analysing file {}", input_files[i].to_str().unwrap());
            ans::analysis::analysis_wrapper(input_files[i].to_str().unwrap(), output_files[i].as_str(),
                                            input_from_minio, &mut box_config, verbose_log);
            println!("file {} analysis, saved at {}", input_files[i].to_str().unwrap(), output_files[i]);
        }
    } else {
        // only specified one output file, then add prefix to each out file.
        if output_files.len() == 1 {
            let output = &output_files[0];
            for input_path in input_files {
                let input_file = input_path.to_str().unwrap();
                let input_path = Path::new(input_file);
                println!("analysing file {}", input_file);
                let output_prefix = input_path.file_name().unwrap().to_str().unwrap();
                let output_file_path = format!("{}-{}", output_prefix, output);
                ans::analysis::analysis_wrapper(input_file, output_file_path.as_str(),
                                                input_from_minio, &mut box_config, verbose_log);
                println!("file {} analysis, saved at {}", input_file, output_file_path.as_str());
            }
        } else {
            println!("files number for input files and output files is no matching ");
            return;
        }
    }

    // todo method
    // todo, now only xyz format input is supported
}

fn mk_parse(format: cli::OutFormat, precision: u32, bin_standard: cli::FormatStandard, ranks: u32, input: &str, output: &str) {
    match format {
        cli::OutFormat::Xyz => {
            binary_parser::parse_wrapper(bin_standard, input, output, ranks, xyz_parser::new_parser(output, precision)).unwrap();
        }
        cli::OutFormat::Text => {
            binary_parser::parse_wrapper(bin_standard, input, output, ranks, text_parser::new_parser(output, precision)).unwrap();
        }
        cli::OutFormat::Dump => {
            binary_parser::parse_wrapper(bin_standard, input, output, ranks, dump_parser::new_parser(output, precision)).unwrap();
        }
    }
}

fn parse_diff(error: f64, file1: String, file2: String, periodic_checking: bool, sim_box: &Vec<f64>) {
    let error_limit: f64 = error;
    let file1: &str = file1.as_str();
    let file2: &str = file2.as_str();

    let mut box_measured_size = (0.0, 0.0, 0.0);
    if !sim_box.is_empty() && sim_box.len() == 3 {
        box_measured_size = (sim_box[0], sim_box[1], sim_box[2]);
    }

    diff::diff::diff_wrapper(file1, file2, error_limit, periodic_checking, box_measured_size);
}
