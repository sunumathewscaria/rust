use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    println!("{:?}", env::current_dir());

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines().map(|l| l.unwrap().to_string()).collect()
}

fn main() {
    let lines = lines_from_file("./src/input.txt");
    let input_length: usize = lines.get(0).unwrap().len();
    let mut ones = vec![0; input_length];
    let mut zeros = vec![0; input_length];
    let mut j: usize = 0;
    let one: char = '1';
    let zero: char = '0';

    let mut oxygen_rating: String = "".to_string();
    let mut scrubber_rating: String = "".to_string();

    let mut oxygen_list: Vec<String> = lines.clone();
    let mut scrubber_list: Vec<String> = lines.clone();

    while j < ones.len() {
        let mut k: usize = 0;
        while k < oxygen_list.len() {
            let diagnostic = oxygen_list.get(k).unwrap().to_string();
            let c: char = diagnostic.chars().nth(j).unwrap();

            if c == (one) {
                ones[j] = ones[j] + 1;
            } else {
                zeros[j] = zeros[j] + 1;
            }

            k = k + 1;
        }
        if oxygen_list.len() == 1 {
            oxygen_rating = oxygen_list.get(0).unwrap().to_string();
        } else {
            if ones[j] > zeros[j] {
                oxygen_rating.push(one);
            } else if ones[j] < zeros[j] {
                oxygen_rating.push(zero);
            } else {
                oxygen_rating.push(one);
            }
        }
        //println!(" oxygenRating {:?}", oxygenRating);
        oxygen_list = oxygen_list
            .clone()
            .into_iter()
            .filter(|l| l.starts_with(&oxygen_rating))
            .collect();

        ones = vec![0; input_length];
        zeros = vec![0; input_length];
        k = 0;
        while k < scrubber_list.len() && scrubber_list.len() > 1 {
            let diagnostic = scrubber_list.get(k).unwrap().to_string();
            let c: char = diagnostic.chars().nth(j).unwrap();

            if c == (one) {
                ones[j] = ones[j] + 1;
            } else {
                zeros[j] = zeros[j] + 1;
            }

            k = k + 1;
        }
        if scrubber_list.len() == 1 {
            scrubber_rating = scrubber_list.get(0).unwrap().to_string();
        } else {
            if ones[j] > zeros[j] {
                scrubber_rating.push(zero);
            } else if ones[j] < zeros[j] {
                scrubber_rating.push(one);
            } else {
                scrubber_rating.push(zero);
            }
        }
        scrubber_list = scrubber_list
            .clone()
            .into_iter()
            .filter(|l| l.starts_with(&scrubber_rating))
            .collect();

        j = j + 1;
        //println!(" scrubberRating {:?}", scrubberRating);

        // for i in scrubberList.clone() {
        //     println!("{:?}", i);
        // }
    }
    println!(" oxygenRating {:?}", oxygen_rating);
    println!(" scrubberRating {:?}", scrubber_rating);
    let e = isize::from_str_radix(&oxygen_rating, 2).unwrap();
    let f = isize::from_str_radix(&scrubber_rating, 2).unwrap();
    println!(" answer  {:?}", (e * f));
}
