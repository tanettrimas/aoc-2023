use std::cmp::Ordering;
use std::collections::HashMap;
use regex::Regex;
use crate::day2::Color::{Blue, Green, Red}; // 0.7.2

use itertools::Itertools;

#[derive(Debug)]
pub struct Game<'a> {
    pub id: u8,
    bag: &'a Bag,
    subsets: Vec<Vec<Cube>>,
}

impl<'a> Game<'a> {
    fn possible(&self, subsets: &Vec<Cube>) -> bool {
        self.bag.contains_all(&subsets)
    }

    pub fn parse(bag: &'a Bag, line: String) -> Self {
        let splits: Vec<_> = line.split(":").collect();
        let metadata = *splits.get(0).expect("Missing a : separator");
        let subset_string = *splits.get(1).expect("Missing a : separator");

        let regex = Regex::new(r"\d+").unwrap();

        let id = regex
            .find(metadata)
            .expect("Should have match")
            .as_str()
            .parse::<u32>()
            .expect("Should be a parseable number");

        let subsets = subset_string.split(";").map(|set| {
            set.split(",").map(|entry| {
                let chunk: Vec<_> = entry.trim().split(" ").collect();
                let color = chunk.last().expect("Color missing in input");
                let amount = chunk.first().expect("Amount missing in input").parse::<usize>().expect("Amount is not parseable");
                Cube(Color::from(color), amount)
            }).collect()
        }).collect();


        Self {
            id: id as u8,
            bag: &bag,
            subsets,
        }
    }

    pub fn is_playable(&self) -> bool {
        let top_cubes = self.top_cubes();
        self.possible(&top_cubes)
    }

    pub fn fewest_cubes_power(&self) -> i64 {
        let top_cubes = self.top_cubes();
        Game::power(&top_cubes)
    }

    fn top_cubes(&self) -> Vec<Cube> {
        let mut hashmap: HashMap<Color, Vec<Cube>> = HashMap::new();
        let mut top_cubes: Vec<Cube> = Vec::new();

        self
            .subsets
            .iter()
            .flatten()
            .for_each(|cube| {
                let x = hashmap.entry(cube.0.clone()).or_insert_with(Vec::new);
                x.push(cube.clone())
            });

        for (_, cubes) in &hashmap {
            let max_cube_by_color = cubes.iter().max().expect("Should have a max cube");
            top_cubes.push(max_cube_by_color.clone());
        }
        top_cubes
    }

    fn power(cubes: &Vec<Cube>) -> i64 {
        let mut sum: i64 = 1;

        cubes.iter().for_each(|cube| {
            sum =  sum * cube.1 as i64
        });

        sum
    }
}

#[derive(Debug)]
pub struct Bag {
    cubes: Vec<Cube>,
}

impl Bag {
    fn new() -> Self {
        Self {
            cubes: Vec::new()
        }
    }

    fn configure(&mut self, configuration_cubes: Vec<Cube>) {
        self.cubes = configuration_cubes;
    }

    pub fn with_cubes(configuration_cubes: Vec<Cube>) -> Self {
        let mut bag = Self::new();
        bag.configure(configuration_cubes);
        bag
    }

    fn contains_all(&self, subsets: &Vec<Cube>) -> bool {
        if subsets.is_empty() {
            return false;
        }

        let mut map = HashMap::new();

        self.cubes.iter().for_each(|cube| {
            map.insert(&cube.0, &cube.1);
            ()
        });
        subsets.iter().all(|set| {
            return match map.get(&set.0) {
                None => false,
                Some(amount) => {
                    return set.1 <= **amount;
                }
            };
        })
    }
    fn sum(cubes: &Vec<Cube>) -> i32 {
        cubes.iter().map(|cube| cube.1 as i32).sum()
    }
}


#[derive(Debug, Clone)]
pub struct Cube(Color, usize);

impl Eq for Cube {}

impl PartialEq<Self> for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd<Self> for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.1.cmp(&other.1))
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl Cube {
    pub(crate) fn new(amount: usize, color: Color) -> Self {
        Self(color, amount)
    }
}


#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub(crate) enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from(str: &str) -> Self {
        match str {
            "red" => Red,
            "blue" => Blue,
            "green" => Green,
            _ => panic!("Unknown color {}", str)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::Color::{Blue, Green, Red};
    use crate::day2::{Bag, Cube, Game};

    #[test]
    fn game_1() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        let game = Game::parse(&bag, "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert_eq!(game.is_playable(), true)
    }

    #[test]
    fn game_2() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        let game = Game::parse(&bag, "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string());

        assert_eq!(game.is_playable(), true)
    }

    #[test]
    fn game_3() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        let game = Game::parse(&bag, "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string());

        assert_eq!(game.is_playable(), false)
    }

    #[test]
    fn game_4() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        let game = Game::parse(&bag, "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string());

        assert_eq!(game.is_playable(), false)
    }

    #[test]
    fn game_5() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        let game = Game::parse(&bag, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string());

        assert_eq!(game.is_playable(), true)
    }

    #[test]
    fn power() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);


        let games: i64 = vec![
            Game::parse(&bag, "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game::parse(&bag, "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()),
            Game::parse(&bag, "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()),
            Game::parse(&bag, "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()),
            Game::parse(&bag, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()),
        ].iter().map(Game::fewest_cubes_power).sum();

        assert_eq!(2286, games)
    }
}

