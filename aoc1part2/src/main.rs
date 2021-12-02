use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};
use std::env;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    println!("{:?}", env::current_dir());
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l|l.unwrap().parse::<i32>().unwrap() )
        .collect()
}

fn main() {
    let lines = lines_from_file("./src/input.txt");
    let mut i: usize = 0;
    let mut count: i32 = 0;
    while i < lines.len() - 3 {
        if sum(&lines, i) < sum(&lines, i+1)  {
            count = count + 1;
        }
        i = i + 1;
    }
    println!("{:?}", count);
}

fn sum(list:&Vec<i32>, i:usize) -> i32 {
    list.get(i).unwrap() + list.get(i+1).unwrap() +list.get(i+2).unwrap()
}