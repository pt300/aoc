use std::io;
use std::io::Read;

fn get_value(input: &str) -> i32 {
    let after_mul = input.split("mul(").skip(1);
    let mut acc: i32 = 0;
    for thing in after_mul {
        if let Some((in_mul, _)) = thing.split_once(")") {
            if let Some((left, right)) = in_mul.split_once(",") {
                acc += left.parse::<i32>().unwrap_or(0) * right.parse::<i32>().unwrap_or(0);
            }
        }       
    }

    acc
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).expect("Input read failed.");

    let solution1 = get_value(&input);
    println!("Problem 1 solution: {}", solution1);
   
    let mut solution2: i32 = 0;
    for do_mul in input.split("do()") {
        if let Some((split, _)) = do_mul.split_once("don't()") {
            solution2 += get_value(split);
        }
        else {
            solution2 += get_value(do_mul);
        }

    }


    println!("Problem 2 solution: {}", solution2);
}
