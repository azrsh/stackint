use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::process;

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
