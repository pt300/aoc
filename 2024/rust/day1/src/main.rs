use std::io;
use std::io::BufRead;
use std::iter::zip;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 { break; }
        let mut ids = line.split_ascii_whitespace();
        
        a.push(ids.next().unwrap().parse().unwrap());
        b.push(ids.next().unwrap().parse().unwrap());
    }

    a.sort_unstable();
    b.sort_unstable();

    let result: i32 = zip(&a, &b).map(|v| (v.0 - v.1).abs()).sum();
    println!("Result part 1: {}", result);

    let result2: i32 = a.iter().map(|v| v * b.iter().filter(|&n| *n == *v).count() as i32).sum();
    println!("Result part 2: {}", result2);
}
