use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let idx: Vec<_> = input.match_indices("move").take(1).collect();
    let terminate = idx[0].0;
    let crates = &input[0..terminate - 2];
    println!("{}", crates);
    let line_vert = crates.lines().count();
    let line_horz: usize = {
        let mut c = 0;
        for n in crates.lines() {
            c = n.len();
            break;
        }
        c
    };

    let cols: Vec<&str> = input[terminate - line_horz - 1..terminate - 3]
        .split_terminator("  ")
        .into_iter()
        .map(|s| s.trim())
        .collect();

    let mut port: HashMap<&str, Vec<String>> = HashMap::new();

    let minus_factor = line_horz + 1;
    for col in &cols {
        let col_pos: Vec<(usize, &str)> = crates.match_indices(col).take(1).collect();
        let mut acc = col_pos[0].0;
        let mut overflow = false;
        for _ in 1..=9 {
            (acc, overflow) = acc.overflowing_sub(minus_factor);
            if overflow {
                break;
            }
            let ch = crates.chars().nth(acc).unwrap_or('w');
            if ch.is_whitespace() {
                break;
            }
            if let Some(containers) = port.get_mut(col) {
                containers.push(ch.to_string());
            } else {
                port.insert(col, vec![ch.to_string()]);
            };
        }
    }

    port.iter()
        .map(|(k, v)| println!("{} : {:?}", k, v))
        .count();

    let instructions: &Vec<&str> = &input[terminate..]
        .split_whitespace()
        .filter_map(|s| {
            if s.parse::<u32>().is_ok() {
                return Some(s);
            }
            return None;
        })
        .collect();
    for w in instructions.windows(3).step_by(3) {
        println!("{:?}", &w);
    }

    for w in instructions.windows(3).step_by(3) {
        let mov = w[0];
        let from = w[1];
        let to = w[2];

        println!("{:?}", w);

        let container = port.get_mut(from).unwrap();
        println!(
            "subtracting {} from {}",
            mov.parse::<usize>().unwrap(),
            container.len()
        );
        let split_at = container
            .len()
            .saturating_sub(mov.parse::<usize>().unwrap());
        let mut moved = container.split_off(split_at);
        moved.reverse();
        println!(
            "split_at: {} {:?} splitting off: {:?}",
            split_at, container, moved
        );
        if let Some(other_container) = port.get_mut(to) {
            other_container.append(&mut moved);
            println!("added to: {:?}", other_container);
        }
    }

    let mut result = vec![];
    for col in &cols {
        result.push(
            port.get(col)
                .unwrap()
                .last()
                .unwrap_or(&"".to_string())
                .to_string(),
        )
    }

    println!("part 1: {:?}", result.join(""))
}
