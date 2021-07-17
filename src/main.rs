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

#[cfg(test)]
mod tests {
    use crate::vm;
    #[test]
    fn add() {
        let mut buf = Vec::<u8>::new();
        vm::interpret(
            &mut buf,
            "push 1\n\
            push 2\n\
            add\n\
            call 0\n\
            halt\n"
                .to_string(),
        );
        assert_eq!(buf, b"3\n");
    }
}
