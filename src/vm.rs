use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::process;

enum Function<'a, W: io::Write> {
    Native(
        fn(
            w: &mut W,
            stack: &mut VecDeque<i32>,
            function_history: &mut VecDeque<usize>,
            variables: &mut HashMap<String, i32>,
            function_table: &Vec<Function<W>>,
        ),
    ),
    Source(&'a str),
}

fn push(stack: &mut VecDeque<i32>, i: i32) {
    stack.push_back(i);
}

fn pop(stack: &mut VecDeque<i32>) -> i32 {
    stack.pop_back().unwrap()
}

fn jump(pc: &mut usize, i: i32) {
    *pc = i as usize;
}

fn jumpif(stack: &mut VecDeque<i32>, pc: &mut usize, i: i32) {
    if pop(stack) == 0 {
        *pc = i as usize;
    }
}

fn add(stack: &mut VecDeque<i32>) {
    let x = pop(stack);
    let y = pop(stack);
    push(stack, x + y);
}

fn sub(stack: &mut VecDeque<i32>) {
    let x = pop(stack);
    let y = pop(stack);
    push(stack, x - y);
}

fn mul(stack: &mut VecDeque<i32>) {
    let x = pop(stack);
    let y = pop(stack);
    push(stack, x * y);
}

fn set(stack: &mut VecDeque<i32>, variables: &mut HashMap<String, i32>, name: String) {
    variables.insert(name, pop(stack));
}

fn get(stack: &mut VecDeque<i32>, variables: &HashMap<String, i32>, name: String) {
    push(stack, variables[&name]);
}

fn call<W: io::Write>(
    w: &mut W,
    stack: &mut VecDeque<i32>,
    function_history: &mut VecDeque<usize>,
    variables: &mut HashMap<String, i32>,
    function_table: &Vec<Function<W>>,
    function_id: usize,
) {
    function_history.push_back(function_id);
    interpret_function(w, stack, function_history, variables, function_table)
}

fn ret(function_history: &mut VecDeque<usize>) {
    function_history.pop_back().unwrap();
}

fn halt() {
    process::exit(0);
}

fn interpret_function<W: io::Write>(
    w: &mut W,
    stack: &mut VecDeque<i32>,
    function_history: &mut VecDeque<usize>,
    variables: &mut HashMap<String, i32>,
    function_table: &Vec<Function<W>>,
) {
    match function_table[*function_history.back().unwrap()] {
        Function::Native(function) => function(w, stack, function_history, variables, function_table),
        Function::Source(source) => {
            let lines = source.split("\n").collect::<Vec<&str>>();
            let mut line_num = 0;
            while line_num < lines.len() {
                let line = lines[line_num];
                line_num += 1;

                let line: Vec<&str> = line.trim_end().split(" ").collect();

                if line[0] == "push" {
                    let i = line[1].parse::<i32>().unwrap();
                    push(stack, i);
                } else if line[0] == "pop" {
                    pop(stack);
                } else if line[0] == "jump" {
                    let i = line[1].parse::<i32>().unwrap();
                    jump(&mut line_num, i);
                } else if line[0] == "jumpif" {
                    let i = line[1].parse::<i32>().unwrap();
                    jumpif(stack, &mut line_num, i);
                } else if line[0] == "add" {
                    add(stack);
                } else if line[0] == "sub" {
                    sub(stack);
                } else if line[0] == "mul" {
                    mul(stack);
                } else if line[0] == "set" {
                    set(stack, variables, line[1].to_string());
                } else if line[0] == "get" {
                    get(stack, variables, line[1].to_string());
                } else if line[0] == "call" {
                    let id = line[1].parse::<usize>().unwrap();
                    call(w, stack, function_history, variables, function_table, id);
                } else if line[0] == "ret" {
                    ret(function_history);
                    return;
                } else if line[0] == "halt" {
                    halt();
                } else {
                    panic!("Unkown code");
                }
            }
        }
    }
}

pub fn interpret<W: io::Write>(w: &mut W, source: String) {
    let mut stack = VecDeque::<i32>::new();
    let mut function_history = VecDeque::<usize>::new();
    let mut variables = HashMap::<String, i32>::new();
    let mut function_table = Vec::<Function<W>>::new();

    function_table.push(Function::Native(
        |w: &mut W,
         stack: &mut VecDeque<i32>,
         _function_history: &mut VecDeque<usize>,
         _variables: &mut HashMap<String, i32>,
         _function_table: &Vec<Function<W>>| {
            let i = *stack.back().unwrap();
            writeln!(w, "{}", i).unwrap();
        },
    ));
    function_table.push(Function::Source(&source));

    call(
        w,
        &mut stack,
        &mut function_history,
        &mut variables,
        &function_table,
        1,
    );
}
