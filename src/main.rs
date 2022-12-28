use clap::{command, Args, Parser, Subcommand};
mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "A React-oriented CLI thought for boosting React productivity.",
    long_about = "React-CLI
A CLI thought for boosting React productivity and avoid boilerplate. 
This comes from the lassitude you can have always installing the same dependencies, creating over and over the same folders, files, routing system, copying hooks, constants, utils, and helper functions from one project to another"
)]

struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Reverses a string
    Reverse(Reverse),
    // Inspects a string
    Inspect(Inspect),
}

#[derive(Args)]
struct Reverse {
    // The string to reverse
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    // The string to inspect
    string: Option<String>,

    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Reverse(name)) => match name.string {
            Some(ref _name) => {
                let reverse = api::rcli::reverse(_name);
                println!("{}", reverse);
            }
            None => {
                println!("Please provide a string to reverse");
            }
        },
        Some(Commands::Inspect(name)) => match name.string {
            Some(ref _name) => {
                let (res, kind) = api::rcli::inspect(_name, name.only_digits);

                let mut plural_s = "s";
                if res == 1 {
                    plural_s = "";
                }

                println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
            }
            None => {
                println!("Please provide a string to inspect");
            }
        },
        None => {}
    }
}
