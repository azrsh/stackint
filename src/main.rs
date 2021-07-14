mod vm;
use crate::vm;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(args[1].clone()).expect("Something went wrong reading the file");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    interpret(&mut stdout, contents);
}

fn interpret<W: io::Write>(w: &mut W, source: String) {
    let mut stack = VecDeque::<i32>::new();
    let mut variables = HashMap::<String, i32>::new();

    let lines = source.split("\n").collect::<Vec<&str>>();
    let mut line_num = 0;
    while line_num < lines.len() {
        let line = lines[line_num];
        line_num += 1;

        let line: Vec<&str> = line.trim_end().split(" ").collect();

        if line[0] == "push" {
            let i = line[1].parse::<i32>().unwrap();
            vm::push(&mut stack, i);
        } else if line[0] == "pop" {
            vm::pop(&mut stack);
        } else if line[0] == "jump" {
            let i = line[1].parse::<i32>().unwrap();
            vm::jump(&mut line_num, i);
        } else if line[0] == "jumpif" {
            let i = line[1].parse::<i32>().unwrap();
            vm::jumpif(&mut stack, &mut line_num, i);
        } else if line[0] == "add" {
            vm::add(&mut stack);
        } else if line[0] == "sub" {
            vm::sub(&mut stack);
        } else if line[0] == "mul" {
            vm::mul(&mut stack);
        } else if line[0] == "set" {
            vm::set(&mut stack, &mut variables, line[1].to_string());
        } else if line[0] == "get" {
            vm::get(&mut stack, &variables, line[1].to_string());
        } else if line[0] == "print" {
            vm::print(&mut stack, w);
        } else if line[0] == "halt" {
            vm::halt();
        } else {
            panic!("Unkown code");
        }
    }
}

#[cfg(test)]
mod tests {
use crate::interpret;
    #[test]
    fn add() {
        let mut buf = Vec::<u8>::new();
        interpret(
            &mut buf,
            "push 1\n\
            push 2\n\
            add\n\
            print\n\
            halt\n".to_string(),
        );
        assert_eq!(buf, b"3\n");
    }
}
