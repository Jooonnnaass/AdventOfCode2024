use std::{fs::File, i32, io::Read};

fn main() {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let mut string = String::new();

    let _ = File::open("inputs/inputone.txt")
        .expect("Could not open inputone.txt")
        .read_to_string(&mut string);

    for line in string.lines() {
        let mut line_iterator = line.split_whitespace();
        left.push(line_iterator.next().unwrap().parse::<i32>().unwrap());
        right.push(line_iterator.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = left
        .iter()
        .zip(&right)
        .fold(0, |acc, n| acc + n.0.abs_diff(*n.1) as i32);

    println!("{}", result);

    let result: i32 = left.iter().fold(0, |acc, l| {
        acc + l * right.iter().filter(|r| *r == l).count() as i32
    });

    println!("{}", result);
}
