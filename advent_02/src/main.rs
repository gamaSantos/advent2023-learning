use std::fs;

#[derive(Debug)]
struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl GameSet {
    fn is_possible(&self, r: i32, g: i32, b: i32) -> bool {
        return self.red <= r && self.green <= g && self.blue <= b;
    }

    fn get_power(&self) -> i32 {
        return self.red * self.green * self.blue;
    }
}

fn main() {
    let file_path = "advent2023-learning/advent_02/input.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let records = input.split('\n');
    let result = get_min_power(records);
    println!("{result}");
}

fn get_min_power(records: std::str::Split<'_, char>) -> i32 {
    let mut sum = 0;
    for game_record in records {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let column_position = game_record.find(':').expect("invalid input");
        let record = game_record[column_position + 1..].trim();
        let sets = record.split(';');
        for set in sets {
            let game_set = map_set(set);
            if game_set.red > max_red {
                max_red = game_set.red;
            }
            if game_set.green > max_green {
                max_green = game_set.green;
            }
            if game_set.blue > max_blue {
                max_blue = game_set.blue;
            }
        }
        let min_set = GameSet {
            red: max_red,
            green: max_green,
            blue: max_blue,
        };
        sum += min_set.get_power();
    }
    return sum;
}

fn get_sum_possible_ids(records: std::str::Split<'_, char>) -> i32 {
    let mut sum = 0;
    for game_record in records {
        let mut is_game_possible = true;
        let column_position = game_record.find(':').expect("invalid input");
        let id = game_record[5..column_position]
            .trim()
            .parse::<i32>()
            .unwrap();
        let record = game_record[column_position + 1..].trim();
        let sets = record.split(';');
        for set in sets {
            let game_set = map_set(set);
            if game_set.is_possible(12, 13, 14) == false {
                is_game_possible = false;
                break;
            }
        }
        if is_game_possible {
            sum += id;
        }
    }
    sum
}

fn map_set(set_text: &str) -> GameSet {
    let cubes = set_text.trim().split(',');
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for cube_identifier in cubes {
        let normalized_identifier = cube_identifier.trim();
        let space_pos = normalized_identifier.find(' ').expect("invalid format");

        let qty = normalized_identifier[..space_pos]
            .trim()
            .parse::<i32>()
            .unwrap();
        let color_id = normalized_identifier[space_pos + 1..space_pos + 2].trim();
        match color_id {
            "r" => r = qty,
            "g" => g = qty,
            "b" => b = qty,
            _ => continue,
        }
    }

    return GameSet {
        red: r,
        green: g,
        blue: b,
    };
}
