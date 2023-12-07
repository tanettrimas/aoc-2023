use std::collections::HashMap;
use std::num::ParseIntError;

const word_to_num_binding: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub fn find_calibration(hay: &str) -> Result<i32, ParseIntError> {
    let number_and_word_mapping = [
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ].iter().chain(&word_to_num_binding);

    let mut word_indices: Vec<(usize, &str)> = Vec::new();

    let map: HashMap<_, _> = number_and_word_mapping.map(|item| *item).collect();


    &map
        .iter().for_each(|(key, _)| {
        hay.match_indices(key).collect::<Vec<_>>().iter().for_each(|element| word_indices.push(*element));
    });

    word_indices.
        sort_by(|(a, _), (b, _)| a.cmp(&b));


    let first_word_or_number = word_indices.first().map(|(_, key)| {
        *map.get(key).expect("Should have a first value")
    }).unwrap();

    let last_word_or_number = word_indices.last().map(|(_, key)| {
        *map.get(key).unwrap_or(&first_word_or_number)
    }).unwrap();

    format!("{}{}", &first_word_or_number, &last_word_or_number).parse::<i32>()
}

#[cfg(test)]
mod test {
    use crate::day1::find_calibration;

    #[test]
    fn test_find_calibration() {
        assert_eq!(find_calibration("1abc2"), Ok(12));
        assert_eq!(find_calibration("twone"), Ok(21));
        assert_eq!(find_calibration("29787jbhkhtbnbgfoursixfour"), Ok(24));
        assert_eq!(find_calibration("pqr3stu8vwx"), Ok(38));
        assert_eq!(find_calibration("a1b2c3d4e5f"), Ok(15));
        assert_eq!(find_calibration("treb7uchet"), Ok(77));
        assert_eq!(find_calibration("two1nine"), Ok(29));
        assert_eq!(find_calibration("eightwothree"), Ok(83));
        assert_eq!(find_calibration("abcone2threexyz"), Ok(13));
        assert_eq!(find_calibration("4nineeightseven2"), Ok(42));
        assert_eq!(find_calibration("zoneight234"), Ok(14));
        assert_eq!(find_calibration("7pqrstsixteen"), Ok(76));

        assert_eq!(vec![
            find_calibration("two1nine").unwrap(),
            find_calibration("xtwone3four").unwrap(),
            find_calibration("eightwothree").unwrap(),
            find_calibration("abcone2threexyz").unwrap(),
            find_calibration("4nineeightseven2").unwrap(),
            find_calibration("zoneight234").unwrap(),
            find_calibration("7pqrstsixteen").unwrap(),
        ].iter().sum::<i32>(), 281)
    }
}