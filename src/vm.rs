use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::process;

struct RuntimeContext<'a, W: io::Write> {
    w: &'a mut W,
    stack: VecDeque<i32>,
    function_history: VecDeque<u32>,
    variables: HashMap<String, i32>,
    function_table: &'a HashMap<u32, Function<'a, W>>,
}

impl<'a, W: io::Write> RuntimeContext<'a, W> {
    fn new(w: &'a mut W, function_table: &'a HashMap<u32, Function<'a, W>>) -> Self {
        Self {
            w,
            stack: VecDeque::new(),
            function_history: VecDeque::new(),
            variables: HashMap::new(),
            function_table,
        }
    }
}

struct FunctionCallContext {
    line_num: usize,
}

enum Function<'a, W: io::Write> {
    Native(fn(ctx: &mut RuntimeContext<'a, W>)),
    Source(&'a [&'a str]),
}

fn push(stack: &mut VecDeque<i32>, i: i32) {
    stack.push_back(i);
}

fn pop(stack: &mut VecDeque<i32>) -> i32 {
    stack.pop_back().unwrap()
}

fn jump(_pc: &mut usize, _i: i32) {
    // *pc = i as usize;
    unimplemented!();
}

fn jumpif(_stack: &mut VecDeque<i32>, _pc: &mut usize, _i: i32) {
    // if pop(stack) == 0 {
    //    *pc = i as usize;
    // }
    unimplemented!();
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

fn set(_stack: &mut VecDeque<i32>, _variables: &mut HashMap<String, i32>, _name: String) {
    // variables.insert(name, pop(stack));
    unimplemented!();
}

fn get(_stack: &mut VecDeque<i32>, _variables: &HashMap<String, i32>, _name: String) {
    // push(stack, variables[&name]);
    unimplemented!();
}

fn call<W: io::Write>(ctx: &mut RuntimeContext<W>, function_id: u32) {
    ctx.function_history.push_back(function_id);
    match ctx.function_table[ctx.function_history.back().unwrap()] {
        Function::Native(function) => function(ctx),
        Function::Source(source) => interpret_function(ctx, source),
    }
}

fn ret(function_history: &mut VecDeque<u32>) {
    function_history.pop_back().unwrap();
}

fn halt() {
    process::exit(0);
}

fn interpret_instruction<W: io::Write>(
    ctx: &mut RuntimeContext<W>,
    fn_ctx: &mut FunctionCallContext,
    instruction: &str,
) {
    let line: Vec<&str> = instruction.trim().split_whitespace().collect();

    if line[0] == "push" {
        let i = line[1].parse::<i32>().unwrap();
        push(&mut ctx.stack, i);
    } else if line[0] == "pop" {
        pop(&mut ctx.stack);
    } else if line[0] == "jump" {
        let i = line[1].parse::<i32>().unwrap();
        jump(&mut fn_ctx.line_num, i);
    } else if line[0] == "jumpif" {
        let i = line[1].parse::<i32>().unwrap();
        jumpif(&mut ctx.stack, &mut fn_ctx.line_num, i);
    } else if line[0] == "add" {
        add(&mut ctx.stack);
    } else if line[0] == "sub" {
        sub(&mut ctx.stack);
    } else if line[0] == "mul" {
        mul(&mut ctx.stack);
    } else if line[0] == "set" {
        set(&mut ctx.stack, &mut ctx.variables, line[1].to_string());
    } else if line[0] == "get" {
        get(&mut ctx.stack, &mut ctx.variables, line[1].to_string());
    } else if line[0] == "call" {
        let id = line[1].parse::<u32>().unwrap();
        call(ctx, id);
    } else if line[0] == "ret" {
        ret(&mut ctx.function_history);
        return;
    } else if line[0] == "halt" {
        halt();
    } else {
        panic!("Unkown code");
    }
}

fn interpret_function<W: io::Write>(runtime_ctx: &mut RuntimeContext<W>, instructions: &[&str]) {
    let mut ctx = FunctionCallContext { line_num: 0 };
    while ctx.line_num < instructions.len() {
        let instruction = instructions[ctx.line_num];
        ctx.line_num += 1;

        interpret_instruction(runtime_ctx, &mut ctx, instruction);
    }
}

pub fn interpret<W: io::Write>(w: &mut W, source: String) {
    let mut function_table = HashMap::<u32, Function<W>>::new();

    let lines = source.split('\n').collect::<Vec<&str>>();
    let functions: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i, str::trim(line)))
        .filter_map(|(i, line)| {
            if line.starts_with("fn") && line.ends_with(":") {
                let match_trimmed = &line[("fn".len())..(line.len() - ":".len())];
                let name = match_trimmed.trim();
                Some((i, name))
            } else {
                None
            }
        })
        .chain([(lines.len(), "")])
        .collect();

    for pair in functions.windows(2) {
        let prev = pair[0];
        let next = pair[1];

        function_table.insert(1, Function::Source(&lines[(prev.0 + 1)..next.0]));
    }

    function_table.insert(
        0,
        Function::Native(|ctx: &mut RuntimeContext<W>| {
            let i = *ctx.stack.back().unwrap();
            writeln!(ctx.w, "{}", i).unwrap();
        }),
    );

    call(&mut RuntimeContext::new(w, &function_table), 1);
}

#[cfg(test)]
mod tests {
    use super::interpret;
    #[test]
    fn add() {
        let mut buf = Vec::<u8>::new();
        interpret(
            &mut buf,
            r#"fn main:
            push 1
            push 2
            add
            call print
            halt
            "#
            .to_string(),
        );
        assert_eq!(buf, b"3\n");
    }
}
