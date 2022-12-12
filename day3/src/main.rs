fn main() {
    // part 1
    let input = include_str!("../input.txt");
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
    let sum: usize = items
        .iter()
        .map(|c| {
            if c.is_ascii_uppercase() {
                println!("uppercase char: {} {}", c, c.to_owned() as usize - 38);
                return c.to_owned() as usize - 38;
            }
            println!("lowercase char: {} {}", c, c.to_owned() as usize - 38);
            return c.to_owned() as usize - 96;
        })
        .sum();
    println!("{:?}: {}", items, sum);
    // input.chars().map(|c| {});
}
