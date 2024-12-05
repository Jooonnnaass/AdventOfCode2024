use std::{fs::File, io::Read};

fn main() {
    let mut string: String = String::new();
    let _ = File::open("inputs/inputone.txt")
        .expect("Could not open inputone.txt")
        .read_to_string(&mut string);
    let vec: Vec<i32> = string
        .split("\n")
        .map(|n| {
            if n.contains(" ") {
                return n.parse::<i32>().unwrap();
            }
            0
        })
        .collect();

    let mut left_list: Vec<&i32> = Vec::new();
    let mut right_list: Vec<&i32> = Vec::new();

    for (i, str) in vec.iter().enumerate() {
        if i % 2 == 0 {
            left_list.push(str);
        } else {
            right_list.push(str);
        }
    }

    println!("{}", left_list.len());
    println!("{:?}", left_list);
    println!("{}", right_list.len());
    println!("{:?}", right_list);
}
