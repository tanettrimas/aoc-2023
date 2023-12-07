use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use crate::day1::find_calibration;

mod day1;

fn main() {
    day1().expect("Something went wrong");
}

fn get_path(day: i32) -> PathBuf {
    let binding = env::current_dir().unwrap();
    let current_directory = binding.to_str();
    let path_str = current_directory.map(|v| format!("{}/src/resources/day{}.txt", v, day)).unwrap();
    let path = Path::new(&path_str);
    path.to_path_buf()
}

fn day1() -> io::Result<()> {
    let path = get_path(1);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sum: i32 = reader
        .lines()
        .map(|line| {
            let current_line = line.unwrap();
            let calibration_result = find_calibration(&current_line);
            calibration_result.unwrap()
        }).sum();
    println!("All calibrations: {}", sum);
    Ok(())
}