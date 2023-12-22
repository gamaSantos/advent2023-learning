use std::{env, fs, path::Path};

use card::Card;

mod card;

fn main() {
    let maybe_file_path = env::args().nth(1);
    let file_path_text = match maybe_file_path {
        Some(f) => f,
        None => panic!("please provide the input file path"),
    };
    let file_path = Path::new(&file_path_text);
    if file_path.exists() == false {
        panic!("Could not find {file_path_text}");
    }

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let result = sum_points(&input);
    println!("sum of points: {result}");
}

fn sum_points(input: &str) -> u64 {
    let lines = input.split('\n');
    let cards = lines.map(|row| Card::from_text(row));
    let mut result = 0;
    for card in cards {
        result = result + card.calculate_points();
    }
    return result;
}

mod tests {
    use crate::sum_points;

    #[test]
    fn acceptance_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let actual = sum_points(input);
        assert_eq!(actual, 13);
    }
}
