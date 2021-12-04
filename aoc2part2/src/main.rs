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
    aim: i32,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Movement> {
    println!("{:?}", env::current_dir());

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().split(' ').map(|l| l.to_string())
            .collect::<Vec<String>>())
        .map(|l| Movement {
            direction: l.get(0).unwrap().to_string(),
            steps: l.get(1).unwrap().parse::<i32>().unwrap(),
        })
        .collect()
}

fn main() {
    let lines = lines_from_file("./src/input.txt");
    let mut i: usize = 0;
    let mut pos = Position { horizontal: 0, depth: 0, aim: 0 };
    while i < lines.len() {
        let movement = lines.get(i).unwrap();
        let strove = movement.direction.to_string();
        match strove.as_str() {
            "forward" => {
                pos.horizontal = pos.horizontal + movement.steps;
                pos.depth = pos.depth + pos.aim * movement.steps
            }
            "up" => {
                pos.aim = pos.aim - movement.steps
            }
            "down" => {
                pos.aim = pos.aim + movement.steps
            }
            _ => { println!("{}", movement.direction) }
        }
        //println!("{:?}", pos);
        i = i + 1;
    }
    println!("{:?}", pos.horizontal * pos.depth);
}

