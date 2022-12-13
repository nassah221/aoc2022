use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    // for line in input.lines() {
    let vec_char = input.chars().collect::<Vec<char>>();

    let first_match = vec_char
        .as_slice()
        .windows(4)
        .enumerate()
        .filter_map(|(i, w)| {
            // If the size of a set is equal to the window size, then we have only unique
            // entries. There is no other way.
            if HashSet::<char>::from_iter(w.to_vec().into_iter()).len() == 4 {
                Some(i)
            } else {
                None
            }
        })
        .take(1)
        .next()
        .unwrap();

    println!("part 1: {}", first_match + 4)
}
