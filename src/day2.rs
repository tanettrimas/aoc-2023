use std::collections::HashMap;
use regex::Regex;
use crate::day2::Color::{Blue, Green, Red}; // 0.7.2


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

        let subsets= subset_string.split(";").map(|set| {
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
        self.subsets.iter().all(|subset| self.possible(&subset))
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
}


#[derive(Debug)]
pub struct Cube(Color, usize);

impl Cube {
    pub(crate) fn new(amount: usize, color: Color) -> Self {
        Self(color, amount)
    }
}


#[derive(Eq, PartialEq, Hash, Debug)]
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
}

