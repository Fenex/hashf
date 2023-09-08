use ::clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short = 'N', help = "How many tail zeroes should hash contains.")]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=64))]
    pub count_zeroes: u8,
    #[clap(short = 'F', help = "How many hashes required to be find.")]
    pub records: usize,
    #[clap(
        short = 't',
        long,
        help = "Number of threads. Defaults is equal to number of logical CPU cores."
    )]
    pub threads: Option<usize>,
    #[clap(long, help = "Enable trace messages via setting up RUST_LOG env.")]
    pub debug: bool,
}
