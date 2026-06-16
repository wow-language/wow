mod ast;
mod backends;
mod checker;
mod lexer;
mod parser;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
use std::process;

/// The runtimes + auzaar toolbox, baked into the compiler. We write the right
/// one next to every generated file so a wow program runs with no external setup.
const AUZAAR_H: &str = include_str!("../runtime/auzaar/auzaar.h");
const AUZAAR_JS: &str = include_str!("../runtime/auzaar/auzaar.js");
const AUZAAR_ARDUINO_H: &str = include_str!("../runtime/auzaar/auzaar_arduino.h");

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

    // Write the auzaar runtime next to the output so it runs with no setup.
    match target {
        Target::C => write_runtime(&out_path, "auzaar.h", AUZAAR_H),
        Target::Node => write_runtime(&out_path, "auzaar.js", AUZAAR_JS),
        Target::Arduino => write_runtime(&out_path, "auzaar_arduino.h", AUZAAR_ARDUINO_H),
    }

    println!("Tayyar: {}", out_path.display());

    if run {
        run_output(&out_path, target);
    }
}

/// Write a bundled runtime file (auzaar.h / auzaar.js) next to the output.
fn write_runtime(out_path: &PathBuf, name: &str, contents: &str) {
    let runtime = out_path.with_file_name(name);
    fs::write(&runtime, contents).unwrap_or_else(|_| {
        eprintln!("Ghalti: runtime ({name}) nahi likh saka");
        process::exit(1);
    });
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
            // Compile with gcc (-lm for the math functions) then run.
            let binary = path.with_extension("");
            let status = Command::new("gcc")
                .args([path.to_str().unwrap(), "-o", binary.to_str().unwrap(), "-lm"])
                .status();
            match status {
                Ok(s) if s.success() => {
                    Command::new(&binary).status().ok();
                }
                _ => {
                    eprintln!("Ghalti: gcc se compile nahi ho saka");
                    process::exit(1);
                }
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
