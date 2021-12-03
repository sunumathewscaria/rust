use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};
use std::env;

#[derive(Debug)]
struct Movement {
    direction: String,
    steps: i32,
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Movement> {
    println!("{:?}", env::current_dir());

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().split(' ').map(|l| l.to_string()).collect::<Vec<String>>())
        .map(|l| Movement { direction: l.get(0).unwrap().to_string(), steps: l.get(1).unwrap().parse::<i32>().unwrap() })
        .collect()
}

fn main() {
    let lines = lines_from_file("./src/input.txt");
    let mut i: usize = 0;
    let mut pos = Position { horizontal: 0, depth: 0 };
    while i < lines.len()  {
        let strove = lines.get(i).unwrap().direction.to_string();
        match strove.as_str() {
            "forward" => { pos.horizontal = pos.horizontal + lines.get(i).unwrap().steps }
            "up" => { pos.depth = pos.depth - lines.get(i).unwrap().steps }
            "down" => { pos.depth = pos.depth + lines.get(i).unwrap().steps }
            _ => { println!("{}", lines.get(i).unwrap().direction) }
        }
        //println!("{:?}", pos);
        i = i + 1;
    }
    println!("{:?}", pos.horizontal * pos.depth);

}

