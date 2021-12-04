use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};
use std::env;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    println!("{:?}", env::current_dir());

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.unwrap().to_string()).collect()
}

fn main() {
    let lines = lines_from_file("./src/input.txt");
    let input_length: usize = lines.get(0).unwrap().len();
    let mut ones = vec![0; input_length];
    let mut zeros = vec![0; input_length];
    let mut j: usize = 0;
    let one: char = '1';
    let zero: char = '0';

    while j < lines.len() {
        let diagnostic = lines.get(j).unwrap().to_string();
        for (i, c) in diagnostic.chars().enumerate() {
            if c == (one) {
                ones[i] = ones[i] + 1;
            } else {
                zeros[i] = zeros[i] + 1;
            }
        }
        j = j + 1;
    }

    j = 0;
    let mut gamma_rate: String = "".to_string();
    let mut epsilon_rate: String = "".to_string();


    while j < ones.len() {
        if ones[j] > zeros[j] {
            gamma_rate.push(one);
            epsilon_rate.push(zero);
        } else {
            gamma_rate.push(zero);
            epsilon_rate.push(one);
        }
        j = j + 1;
    }

    println!("{:?}", gamma_rate);
    println!("{:?}", epsilon_rate);

    let gamma_rate_val = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate_val = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    println!("{:?}", gamma_rate_val);
    println!("{:?}", epsilon_rate_val);

    println!("{:?}", gamma_rate_val * epsilon_rate_val);
}

