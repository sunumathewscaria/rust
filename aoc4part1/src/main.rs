use array2d::Array2D;
use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    //AsRef ? what is the use
    println!("{:?}", env::current_dir());

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines().map(|l| l.unwrap().to_string()).collect()
}

fn main() {
    let lines = lines_from_file("./src/input.txt");
    let drawn_numbers = lines[0]
        .split(",")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let vec_array2d = prepare_2darray(lines);

    let mut i = 5; // atleast 5 number is required to be equal to column/row of matrix
    let mut not_found: bool = true;

    while i < drawn_numbers.len() {
        let dst = &drawn_numbers[0..i]; //increasing the array length one by one
        let mut j = 0;
        while j < vec_array2d.len() {
            let array2d = vec_array2d.get(j).unwrap();
            let rows = array2d.as_rows(); // temp variable created here to prevent it being dropped
            let matching_rows = rows
                .iter()
                .filter(|row| row.iter().all(|f| dst.contains(&f)))
                .collect::<Vec<&Vec<String>>>();

            not_found = matching_rows.is_empty();

            if not_found {
                let columns = array2d.as_columns();

                let matching_columns = columns
                    .iter()
                    .filter(|row| row.iter().all(|f| dst.contains(&f)))
                    .collect::<Vec<&Vec<String>>>();
                let non_matching_columns = columns
                    .iter()
                    .filter(|row| !row.iter().all(|f| dst.contains(&f)))
                    .collect::<Vec<&Vec<String>>>();
                not_found = matching_columns.is_empty();
                if !not_found {
                    let flat_map = non_matching_columns
                        .iter()
                        .map(|f| f.iter().map(|f| f))
                        .flat_map(|f| f)
                        .collect::<Vec<&String>>();
                    let sum_value: u32 = flat_map
                        .iter()
                        .filter(|f| !dst.contains(&f))
                        .map(|f| f.parse::<u32>().unwrap())
                        .sum();
                    let lastele = dst.get(i - 1).unwrap().parse::<u32>().unwrap();
                    println!("{:?}", sum_value * &lastele);

                    break;
                }
            } else {
                let non_matching_columns = rows
                    .iter()
                    .filter(|row| !row.iter().all(|f| dst.contains(&f)))
                    .collect::<Vec<&Vec<String>>>();

                let flat_map = non_matching_columns
                    .iter()
                    .map(|f| f.iter().map(|f| f))
                    .flat_map(|f| f)
                    .collect::<Vec<&String>>();
                let sum_value: u32 = flat_map
                    .iter()
                    .filter(|f| !dst.contains(&f))
                    .map(|f| f.parse::<u32>().unwrap())
                    .sum();
                let lastele = dst.get(i - 1).unwrap().parse::<u32>().unwrap();
                println!("{:?}", sum_value * &lastele);
                break;
            }
            j = j + 1;
        }
        if !not_found {
            break;
        }

        i = i + 1;
    }
}

fn prepare_2darray(lines: Vec<String>) -> Vec<Array2D<String>> {
    let mut i = 2;
    let mut vec_array2d = Vec::new();
    while i < lines.len() {
        let first_row = lines[i]
            .split_whitespace()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let second_row = lines[i + 1]
            .split_whitespace()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let third_row = lines[i + 2]
            .split_whitespace()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let fouth_row = lines[i + 3]
            .split_whitespace()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let fifth_row = lines[i + 4]
            .split_whitespace()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let rows = vec![first_row, second_row, third_row, fouth_row, fifth_row];

        let array = Array2D::from_rows(&rows);
        vec_array2d.push(array);
        i = i + 6; // increase by 6 to move to next matrix
    }
    vec_array2d
}
