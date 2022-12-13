fn main() {
    let input = include_str!("../input.txt");
    let count = input
        .lines()
        .filter_map(|line| {
            let section = line.split_once(",").unwrap();
            let mut pair_1 = section.0.splitn(2, "-");
            let mut pair_2 = section.1.splitn(2, "-");

            let (p1x, p1y) = (
                pair_1.next().unwrap().parse::<usize>().unwrap(),
                pair_1.next().unwrap().parse::<usize>().unwrap(),
            );
            let (p2x, p2y) = (
                pair_2.next().unwrap().parse::<usize>().unwrap(),
                pair_2.next().unwrap().parse::<usize>().unwrap(),
            );

            if (p1x <= p2x && p2y <= p1y) || (p2x <= p1x && p1y <= p2y) {
                return Some(1);
            }
            return None;
        })
        .count();
    println!("{}", count)
}
