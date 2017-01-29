use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().next().unwrap().unwrap();

    let mut pairs: Vec<(usize, Option<usize>)> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            pairs.push((i, None));
        } else if c == ')' {
            match pairs.iter_mut().rev().find(|&&mut x| x.1.is_none()) {
                Some(e) => e.1 = Some(i),
                None => {
                    display_extra(&input, i);
                    return;
                }
            }
        }
    }

    for pair in pairs {
        if pair.1.is_none() {
            display_extra(&input, pair.0);
            return;
        }
    }
}

fn display_extra(s: &String, pos: usize) {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == pos {
            result.push('*');
        }
        result.push(c);
    }

    println!("{}", result);
}
