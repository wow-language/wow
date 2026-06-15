mod ast;
mod backends;
mod checker;
mod lexer;
mod parser;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
use std::process;

/// wow — Roman Urdu programming language
#[derive(Parser)]
#[command(name = "wow")]
#[command(version = "0.1.0")]
#[command(about = "Code likho. Wow bolo.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compile a .wow file
    Build {
        /// The source file to compile
        file: PathBuf,

        /// Target platform
        #[arg(long, short, default_value = "c")]
        target: Target,

        /// Also run after compiling
        #[arg(long, short)]
        run: bool,
    },

    /// Compile and immediately run a .wow file
    Run {
        /// The source file to compile and run
        file: PathBuf,

        /// Target platform
        #[arg(long, short, default_value = "c")]
        target: Target,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Target {
    /// Compile to C and run with gcc
    C,
    /// Compile to an Arduino .ino sketch
    Arduino,
    /// Compile to Node.js JavaScript
    Node,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Build { file, target, run } => {
            compile(&file, &target, run);
        }
        Command::Run { file, target } => {
            compile(&file, &target, true);
        }
    }
}

fn compile(file: &PathBuf, target: &Target, run: bool) {
    // Read source
    let source = fs::read_to_string(file).unwrap_or_else(|_| {
        eprintln!("Ghalti: file nahi mila — {:?}", file);
        process::exit(1);
    });

    let filename = file.to_string_lossy().to_string();

    // Lex
    let tokens = lexer::lex(&source, &filename);

    // Parse
    let ast = parser::parse(tokens, &source, &filename);

    // Check (validate keywords are legal for this target, etc.)
    checker::check(&ast, target, &source, &filename);

    // Generate output
    let output = match target {
        Target::C       => backends::c::generate(&ast),
        Target::Arduino => backends::arduino::generate(&ast),
        Target::Node    => backends::node::generate(&ast),
    };

    // Write output file
    let out_path = output_path(file, target);
    fs::write(&out_path, output).unwrap_or_else(|_| {
        eprintln!("Ghalti: file nahi likh saka — {:?}", out_path);
        process::exit(1);
    });

    println!("Tayyar: {:?}", out_path);

    if run {
        run_output(&out_path, target);
    }
}

fn output_path(source: &PathBuf, target: &Target) -> PathBuf {
    let stem = source.file_stem().unwrap_or_default();
    let ext = match target {
        Target::C       => "c",
        Target::Arduino => "ino",
        Target::Node    => "js",
    };
    source.with_file_name(stem).with_extension(ext)
}

fn run_output(path: &PathBuf, target: &Target) {
    use std::process::Command;
    match target {
        Target::C => {
            // Compile with gcc then run
            let binary = path.with_extension("");
            let status = Command::new("gcc")
                .args([path.to_str().unwrap(), "-o", binary.to_str().unwrap()])
                .status();
            if status.map(|s| s.success()).unwrap_or(false) {
                Command::new(&binary).status().ok();
            }
        }
        Target::Arduino => {
            // arduino-cli handles flashing
            Command::new("arduino-cli")
                .args(["compile", "--upload", path.to_str().unwrap()])
                .status()
                .ok();
        }
        Target::Node => {
            Command::new("node")
                .arg(path.to_str().unwrap())
                .status()
                .ok();
        }
    }
}
