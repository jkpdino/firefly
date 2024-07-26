use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub files: Vec<String>,

    /// Print the HIR tree to the console
    #[arg(long)]
    pub print_hir: bool
}