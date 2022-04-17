use std::path;
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version)]
#[clap(arg_required_else_help = true, arg_required_else_help = true)]
#[clap(name = "md-tools")]
#[clap(about = "MISA-MD tools, compatible with MISA-MD (or Crystal MD) v0.3.x, v0.4.x and later versions.")]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum OutFormat {
    Xyz,
    Text,
    Dump,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum FormatStandard {
    Current,
    Next,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum AnsAlgorithm {
    WS,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// convert to tex or xyz format.
    #[clap(arg_required_else_help = true)]
    #[clap(about = "convert binary MD results to text files.")]
    Conv {
        #[clap(short, long, help = "Do everything except actually process files")]
        dry: bool,
        #[clap(short, long, required = true, multiple_values = true, parse(from_os_str), help = "Sets the filename of input files")]
        input: Vec<path::PathBuf>,
        #[clap(short, long, default_value_t = String::from("md-output"), help = "Sets the filename of output file")]
        output: String,
        #[clap(short, long, required = true, arg_enum, value_name = "FORMAT", help = "output format")]
        format: OutFormat,
        #[clap(short, long, default_value_t = 6, help = "the float number precision")]
        precision: u32,
        #[clap(short, long, arg_enum, default_value_t = FormatStandard::Current, value_name = "STANDARD", help = "binary file standard")] // default value can not be required.
        standard: FormatStandard,
        #[clap(short, long, required = true, value_name = "RANKS", help = "ranks to run the parallel program")]
        ranks: usize,
    },
    /// diff files
    #[clap(arg_required_else_help = true)]
    #[clap(about = "compare particles in two xyz FILES id by id.")]
    Diff {
        #[clap(short, long, default_value_t = 1e-4, help = "max error")]
        error: f64,
        #[clap(required = true, help = "first file path for `diff`")]
        file_1: String,
        #[clap(required = true, help = "second file path for `diff`")]
        file_2: String,
        #[clap(short, long, requires = "box", help = "enable/disable periodic boundary checking while performing `diff`")]
        periodic_checking: bool,
        #[clap(short = 'b', long = "box", group = "box", required = true, multiple_values = true, max_values = 3, min_values = 3, help = "the simulation box length, used for periodic boundary checking.")]
        sim_box: Vec<f64>,
    },
    /// defect analysis
    #[clap(arg_required_else_help = true)]
    #[clap(about = "defect analysis.")]
    Ans {
        #[clap(short, long, multiple_values = true, required = true, parse(from_os_str), help = "Sets the filename of input files")]
        input: Vec<path::PathBuf>,
        #[clap(short, long, multiple_values = true, required = true, help = "Sets the filename of output files")]
        output: Vec<String>,
        #[clap(short, long, help = "show verbose log")]
        verbose: bool,
        #[clap(short = 'M', long = "input-from-minio", help = "Read input files from minio or aws s3 server")]
        input_from_minio: bool,
        #[clap(short = 'S', long = "box-start", multiple_values = true, max_values = 3, min_values = 3, help = "start position of simulation box for construction a perfect lattice box (default auto)")]
        box_start: Vec<f64>,
        #[clap(short, long, multiple_values = true, max_values = 3, min_values = 3, help = "simulation box size. Use auto detection if not specified.")]
        box_size: Vec<u64>,
        #[clap(short, long, arg_enum, default_value_t = AnsAlgorithm::WS, help = "algorithm performing defect analysis (ws).")]
        algorithm: AnsAlgorithm,
    },
    // #[clap(external_subcommand)]
    // External(Vec<OsString>),
}
