use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Size of the cube
    #[arg(short, long, default_value_t = 3)]
    pub size: usize,

    /// Panic on invalid pattern inputs
    #[arg(short, long, default_value_t = false)]
    pub panic: bool,

    /// Don't print the cube :(
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    pub pattern: String,
}
