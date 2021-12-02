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
    while i < lines.len() - 1 {
        if lines.get(i) < lines.get(i + 1) {
            count = count + 1;
        }
        i = i + 1;
    }
    println!("{:?}", count);
    for line in lines {
        //println!("{:?}", line);
    }
}