use std::collections::HashMap;
use rand::seq::SliceRandom; // 0.7.2


struct Game {
    id: u8,
}

impl Game {
    fn possible(bag: &Bag, subsets: Vec<Cube>) -> bool {
        bag.contains_all(subsets)
    }
}

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

    fn contains_all(&self, subsets: Vec<Cube>) -> bool {
        if (subsets.is_empty()) {
            return false
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


struct Cube(Color, usize);

impl Cube {
    fn new(amount: usize, color: Color) -> Self {
        Self(color, amount)
    }
}


#[derive(Eq, PartialEq, Hash, Debug)]
enum Color {
    Red,
    Green,
    Blue,
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


        assert_eq!(Game::possible(&bag, Vec::from([
            Cube::new(3, Blue),
            Cube::new(4, Red),
            Cube::new(1, Red),
            Cube::new(2, Green),
            Cube::new(6, Blue),
            Cube::new(2, Green)
        ])), true)
    }

    #[test]
    fn game_2() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let subsets = Vec::from([
            Cube::new(1, Blue),
            Cube::new(2, Green),
            Cube::new(3, Green),
            Cube::new(4, Blue),
            Cube::new(1, Red),
            Cube::new(1, Green),
            Cube::new(1, Blue)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        assert_eq!(Game::possible(&bag, subsets), true)
    }

    #[test]
    fn game_3() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let subsets = Vec::from([
            Cube::new(8, Green),
            Cube::new(6, Blue),
            Cube::new(20, Red),
            Cube::new(5, Blue),
            Cube::new(4, Red),
            Cube::new(13, Green),
            Cube::new(5, Green),
            Cube::new(1, Red)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        assert_eq!(Game::possible(&bag, subsets), false)

    }

    #[test]
    fn game_4() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let subsets = Vec::from([
            Cube::new(1, Green),
            Cube::new(3, Red),
            Cube::new(6, Blue),
            Cube::new(3, Green),
            Cube::new(6, Red),
            Cube::new(3, Green),
            Cube::new(15, Blue),
            Cube::new(14, Red)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        assert_eq!(Game::possible(&bag, subsets), false)
    }

    #[test]
    fn game_5() {
        let configuration_cubes = Vec::from([
            Cube::new(12, Red),
            Cube::new(13, Green),
            Cube::new(14, Blue)
        ]);

        let subsets = Vec::from([
            Cube::new(6, Red),
            Cube::new(1, Blue),
            Cube::new(3, Green),
            Cube::new(2, Blue),
            Cube::new(1, Red),
            Cube::new(2, Green)
        ]);

        let bag = Bag::with_cubes(configuration_cubes);

        assert_eq!(Game::possible(&bag, subsets), true)
    }

}

