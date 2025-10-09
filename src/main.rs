mod core;
mod cli;
mod compiler;
mod modules;

use cli::{parse_args, print_usage, print_version, handle_result, CompilerMode};
use compiler::compile_file;

fn main() {
    let args = parse_args();

    match args.mode {
        CompilerMode::Compile(filename) => {
            handle_result(compile_file(&filename));
        }
        CompilerMode::Help => {
            print_usage();
        }
        CompilerMode::Version => {
            print_version();
        }
    }
}
