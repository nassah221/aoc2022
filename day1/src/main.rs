fn main() {
    let mut sum = include_str!("../input.txt")
        .split("\n\n")
        .map(|v| {
            v.split("\n")
                .map(|s| str::parse::<u32>(s).ok().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();
    sum.sort_by(|a, b| b.cmp(a));
    println!("first part: {:?}", sum.first().unwrap());

    let top_three = sum.iter().take(3).sum::<u32>();
    println!("second part: {:?}", top_three);
}
