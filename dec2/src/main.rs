use std::fs;

#[derive(Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand::Rock => f.write_str("rock"),
            Hand::Paper => f.write_str("paper"),
            Hand::Scissors => f.write_str("scissors"),
        }
    }
}

impl Hand {
    fn get_score(&self) -> i32 {
        match &self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn competitor(&self) -> Hand {
        match &self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn prey(&self) -> Hand {
        match &self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

enum MatchResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl std::fmt::Display for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchResult::Win => f.write_str("win"),
            MatchResult::Loss => f.write_str("loss"),
            MatchResult::Draw => f.write_str("draw"),
        }
    }
}

impl MatchResult {
    fn get_score(&self) -> i32 {
        match &self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Loss => 0,
        }
    }
}

fn parse_hand(c: char) -> Result<Hand, &'static str> {
    let lower = c.to_ascii_lowercase();
    match lower {
        'a' | 'x' => Ok(Hand::Rock),
        'b' | 'y' => Ok(Hand::Paper),
        'c' | 'z' => Ok(Hand::Scissors),
        _ => Err("invalid hand"),
    }
}

fn parse_result(c: char) -> Result<MatchResult, &'static str> {
    let lower = c.to_ascii_lowercase();
    match lower {
        'x' => Ok(MatchResult::Loss),
        'y' => Ok(MatchResult::Draw),
        'z' => Ok(MatchResult::Win),
        _ => Err("invalid result"),
    }
}

fn result_for_hand(them: &Hand, us: &Hand) -> MatchResult {
    match them {
        Hand::Rock => match us {
            Hand::Paper => MatchResult::Win,
            Hand::Scissors => MatchResult::Loss,
            Hand::Rock => MatchResult::Draw,
        },
        Hand::Paper => match us {
            Hand::Paper => MatchResult::Draw,
            Hand::Scissors => MatchResult::Win,
            Hand::Rock => MatchResult::Loss,
        },
        Hand::Scissors => match us {
            Hand::Paper => MatchResult::Loss,
            Hand::Scissors => MatchResult::Draw,
            Hand::Rock => MatchResult::Win,
        },
    }
}

fn score_for_match(them: Hand, us: Hand) -> i32 {
    let result = result_for_hand(&them, &us);
    return us.get_score() + result.get_score();
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut total_score = 0;

    for line in lines {
        let mut vals = line.split_ascii_whitespace();

        let their_val = vals.next().expect("Should have had at least one item");
        let their_hand = parse_hand(
            their_val
                .chars()
                .next()
                .expect("Should have had at least one char"),
        )
        .expect("Should have been a valid hand");
        let our_in = vals.next().expect("Should have had at least one item");
        let our_result = parse_result(
            our_in
                .chars()
                .next()
                .expect("Should have had at least one char"),
        )
        .expect("Should have been a valid result");
        let mut our_hand;
        match our_result {
            MatchResult::Draw => our_hand = their_hand,
            MatchResult::Loss => our_hand = their_hand.prey(),
            MatchResult::Win => our_hand = their_hand.competitor(),
        }
        let predicted_score = score_for_match(their_hand, our_hand);
        total_score += predicted_score;
    }
    println!("{}", total_score);
}
