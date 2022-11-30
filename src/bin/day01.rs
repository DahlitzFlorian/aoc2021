use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn get_data() -> Vec<u32> {
    let file_path = Path::new("assets/day01_input.txt");
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);
    let mut data: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let number: u32 = match line.unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Failed to convert to int"),
        };
        data.push(number);
    }

    return data;
}

fn part_1() {
    let mut counter: u32 = 0;
    let data = get_data();

    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            counter += 1;
        }
    }

    println!("Result for part 1: {}", counter);
}

fn part_2() {
    let data = get_data();
    let mut counter: u32 = 0;
    let mut i: usize = 2;

    while i < data.len() - 1 {
        let first_window = data[i - 2] + data[i - 1] + data[i];
        let second_window = data[i - 1] + data[i] + data[i + 1];

        if second_window > first_window {
            counter += 1;
        }
        i += 1;
    }

    println!("Result for part 1: {}", counter);
}

fn main() {
    part_1();
    part_2();
}
