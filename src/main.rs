use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use crate::day1::find_calibration;
use crate::day2::{Bag, Color, Cube, Game};

mod day1;
mod day2;

fn main() {
    day2().expect("Something went wrong");
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

fn day2() -> io::Result<()> {
    let configuration_cubes = Vec::from([
        Cube::new(12, Color::Red),
        Cube::new(13, Color::Green),
        Cube::new(14, Color::Blue)
    ]);

    let bag = Bag::with_cubes(configuration_cubes);


    let path = get_path(2);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut sum_of_ids: i32 = 0;
    let mut sum_of_power: i64 = 0;
    reader.lines().for_each(|line_result| {
        let line = line_result.unwrap();
        let game = Game::parse(&bag, line);
        match game.is_playable() {
            true => {
                sum_of_ids += game.id as i32
            }
            false => {}
        }
        sum_of_power += game.fewest_cubes_power();
    });
    println!("Possible games: {}", sum_of_ids);
    println!("Total power: {}", sum_of_power);
    Ok(())
}