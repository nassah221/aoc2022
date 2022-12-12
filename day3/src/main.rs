fn main() {
    let input = include_str!("../input.txt");

    // part 1
    let items = input
        .lines()
        .map(|l| {
            let (compartment1, compartment2) = l.split_at(l.len() / 2);
            let dup = compartment1
                .chars()
                .find_map(|c| compartment2.find(c.clone()));
            compartment2.chars().nth(dup.unwrap())
        })
        .flatten()
        .collect::<Vec<char>>();

    // part 2
    let badges = input
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .step_by(3)
        .map(|w| {
            let vec_1 = w.get(0).unwrap().chars().collect::<Vec<char>>();
            let vec_2 = w.get(1).unwrap().chars().collect::<Vec<char>>();
            let vec_3 = w.get(2).unwrap().chars().collect::<Vec<char>>();

            let common = vec_1
                .iter()
                .filter(|&ch| vec_2.contains(ch) && vec_3.contains(ch))
                .cloned()
                .take(1)
                .collect::<Vec<char>>();

            common
        })
        .flatten()
        .collect::<Vec<char>>();

    println!("{:?}: {}", &items, sum(&items));
    println!("{:?}: {}", &badges, sum(&badges));
}

fn sum(vec: &Vec<char>) -> usize {
    vec.iter()
        .map(|c| {
            if c.is_ascii_uppercase() {
                println!("uppercase char: {} {}", c, c.to_owned() as usize - 38);
                return c.to_owned() as usize - 38;
            }
            println!("lowercase char: {} {}", c, c.to_owned() as usize - 96);
            return c.to_owned() as usize - 96;
        })
        .sum()
}
