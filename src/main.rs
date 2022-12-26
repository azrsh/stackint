mod vm;
use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(args[1].clone()).expect("Something went wrong reading the file");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    vm::interpret(&mut stdout, contents);
}
