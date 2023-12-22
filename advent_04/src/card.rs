pub struct Card {
    winning_numbers: Vec<i32>,
    played_numbers: Vec<i32>,
}

impl Card {
    pub fn from_text(card_text: &str) -> Card {
        let without_id = Card::remove_card_identifier(card_text);
        let splited_numbers = without_id.split('|').collect::<Vec<&str>>();
        let winning_numbers = Card::parse_numbers(splited_numbers[0]);
        let played_numbers = Card::parse_numbers(splited_numbers[1]);
        Card {
            winning_numbers,
            played_numbers,
        }
    }

    pub fn calculate_points(&self) -> u64 {
        let mut matches = 0;
        for played in &self.played_numbers {
            if self.winning_numbers.contains(played) {
                matches = matches + 1;
            }
        }
        if matches == 0 {
            return 0;
        }
        if matches == 1 {
            return 1;
        }
        
        return 2u64.pow(matches - 1);
    }

    fn parse_numbers(text: &str) -> Vec<i32> {
        text.trim().split(' ')
            .filter(|x| x != &"")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }

    fn remove_card_identifier(text: &str) -> &str {
        let sep_digit = text.find(':');
        return match sep_digit {
            Some(x) => &text[x + 1..].trim(),
            None => text,
        };
    }
}

mod tests {
    use super::Card;

    #[test]
    fn remove_card_identifier_should_remove_prefix() {
        let card_text = "Card 0: 1 2 3 | 4 5 6 7";
        let sut = Card::remove_card_identifier(card_text);
        assert_eq!(sut, "1 2 3 | 4 5 6 7");
    }

    #[test]
    fn new_should_parse_values() {
        let card_text = "Card 0: 1 2 3 | 4 5 6 7";
        let sut = Card::from_text(card_text);
        assert_eq!(sut.winning_numbers.len(), 3);
        assert_eq!(sut.played_numbers.len(), 4);
    }

    #[test]
    fn calculate_points_should_be_zero_when_no_match_was_found(){
        let card_text = "Card 0: 1 2 3 | 4 5 6 7";
        let sut = Card::from_text(card_text);
        let actual = sut.calculate_points();
        assert_eq!(actual,0);
    }

    #[test]
    fn calculate_points_should_be_one_when_one_match_was_found(){
        let card_text = "Card 0: 1 2 3 | 3 5 6 7";
        let sut = Card::from_text(card_text);
        let actual = sut.calculate_points();
        assert_eq!(actual,1);
    }

    #[test]
    fn calculate_points_should_be_two_when_two_matches_was_found(){
        let card_text = "Card 0: 1 2 3 | 3 2 6 7";
        let sut = Card::from_text(card_text);
        let actual = sut.calculate_points();
        assert_eq!(actual,2);
    }
}
