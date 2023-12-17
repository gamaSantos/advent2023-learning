use std::fs;
fn main() {
    let file_path = "~/projects/advent2023-learning/advent_01/input";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split('\n');
    let mut sum = 0;
    let accepted_digits = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    for input_line in lines {
        println!("{input_line}");
        let mut first_digit: String = String::from("");
        let mut last_digit = "";
        let mut first_idx = 10000;
        let mut last_idx = 0;

        for dgt in accepted_digits {
            let find_first = input_line.find(dgt);
            let find_last = input_line.rfind(dgt);
            match find_first {
                Some(idx) => {
                    if idx < first_idx {
                        first_idx = idx;
                        first_digit = map_digit(dgt).to_string();
                    }
                    if idx >= last_idx {
                        last_idx = idx;
                        last_digit = map_digit(dgt);
                    }
                    let lidx = find_last.expect("should found last even if it is same");

                    if lidx > last_idx {
                        last_idx = lidx;
                        last_digit = map_digit(dgt);
                    }
                }
                None => continue,
            }
        }
        first_digit.push_str(last_digit);
        println!("{first_digit}");
        sum += first_digit.parse::<i32>().unwrap();
    }
    println!("{sum}");
}

fn map_digit(digit_text: &str) -> &str {
    match digit_text {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => digit_text,
    }
}
