// A X = Rock (1)
// B Y = Paper (2)
// C Z = Scissors (3)

// X(1) - LOSE
// A = C
// B = A
// C = Y

// Z(2) - WIN
//

// [1,2,3]
use anyhow::{anyhow, Ok, Result};

const OUTCOME: [usize; 3] = [0, 3, 6];
const STRATEGY: [usize; 3] = [2, 0, 1];
const TURN: [usize; 3] = [1, 2, 3];

fn get_number(s: &str) -> usize {
    match s {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("invalid input"),
    }
}

fn get_score(s: &str) -> Result<usize> {
    let (o, p) = s.split_once(" ").ok_or(anyhow!("ERROR"))?;
    let (o1, p1) = (get_number(o), get_number(p));
    let outcome = OUTCOME[p1];
    let strategy = STRATEGY[p1];
    let score = TURN[(o1 + strategy) % TURN.len()];

    Ok(score + outcome)
}

fn main() {
    let input = include_str!("../input.txt");

    let score_1 = input
        .clone()
        .split("\n")
        .into_iter()
        .map(|s| s.replace(" ", ""))
        .map(|s| match s.as_str() {
            "AY" => 8,
            "BZ" => 9,
            "CX" => 7,
            "AZ" => 3,
            "BX" => 1,
            "CY" => 2,
            "AX" => 4,
            "BY" => 5,
            "CZ" => 6,
            _ => 0,
        })
        .sum::<u32>();

    let test: usize = input
        .clone()
        .split("\n")
        .into_iter()
        .map(|s| get_score(&s).ok().unwrap())
        .sum();

    println!("part 1: {:?}", score_1);
    println!("part 2: {:?}", test);
}
