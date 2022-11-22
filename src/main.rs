use clap::{Parser, Subcommand};
// #[macro_use]
// extern crate json;

/// workbench - a workspace management cli
#[derive(Debug, Parser)]
#[command(name = "workbench")]
#[command(about = "a workspace manager", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// inits a workbench in the project
  #[command(arg_required_else_help = true)]
  Init {
    /// directory to init in
    #[arg(default_value_t = String::from("./"))]
    dir: String,
  }
}

fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Init { dir } => {

      println!("directory: {}", dir || "./");:
    }
  }
}
