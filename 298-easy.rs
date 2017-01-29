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
                None => println!("No matching opening paren found")
            }
        }
    }

    let pairs: Vec<(usize, usize)> = pairs.iter().map(|&e| (e.0, e.1.unwrap())).collect();

    let mut excluded: Vec<(usize, usize)> = Vec::new();
    let mut iter = pairs.iter().peekable();
    while let Some(pair) = iter.next() {
        match iter.peek() {
            Some(e) => {
                if **e == (pair.0 + 1, pair.1 - 1) {
                    excluded.push(*pair);
                }
            },
            None => break
        };
    }

    let mut chars: Vec<char> = input.chars().collect();
    for (i, pair) in excluded.iter().enumerate() {
        println!("{} - {}", pair.0 - i, pair.1 - (i + 1));
        chars.remove(pair.0 - i);
        chars.remove(pair.1 - (i + 1));
    }

    let result: String = chars.into_iter().collect();
    println!("{}", result);
}
