mod api;
use api::commands::Inspect;
use api::commands::Project;
use api::commands::Reverse;

use clap::{command, Parser, Subcommand};

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
    /// Reverses a string
    Reverse(Reverse),
    /// Inspects a string
    Inspect(Inspect),
    /// Creates a new React project
    Project(Project),
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
        Some(Commands::Project(name)) => match name.project_name {
            Some(ref _name) => {
                let (proj, js, npm) =
                    api::rcli::create_project(_name, name.with_javascript, name.with_npm);
                println!("\nCreating new project {} with React-CLI", proj);
                if !js {
                    println!("Project language: Typescript (default)");
                } else {
                    println!("Project language: Javascript");
                }
                if !npm {
                    println!("Project package manager: Yarn (default)");
                } else {
                    println!("Project package manager: Npm");
                }

                println!("\nDone. Now run:\n");
                println!("   cd {}", proj);
                println!("   yarn install / npm install");
                println!("   yarn dev / npm run dev\n");
                println!("Have fun!\n");
            }
            None => {
                println!("Please provide a project name");
            }
        },
        None => {}
    }
}
