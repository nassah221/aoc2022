use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    // for line in input.lines() {
    let vec_char = input.chars().collect::<Vec<char>>();

    let window_1 = 4;
    let part_1 = find_marker(&vec_char, window_1);

    let window_2 = 14;
    let part_2 = find_marker(&vec_char, window_2);

    println!("part 1: {}", part_1 + window_1);
    println!("part 2 {}", part_2 + window_2)
}

fn find_marker(input: &Vec<char>, window_size: usize) -> usize {
    input
        .as_slice()
        .windows(window_size)
        .enumerate()
        .filter_map(|(i, w)| {
            // If the size of a set is equal to the window size, then we have only unique
            // entries. There is no other way.
            if HashSet::<char>::from_iter(w.to_vec().into_iter()).len() == window_size {
                Some(i)
            } else {
                None
            }
        })
        .take(1)
        .next()
        .unwrap()
}
