use std::collections::HashMap;

#[derive(Debug)]
struct Dir {
    size: usize,
    parent: Option<usize>,
    children: HashMap<&'static str, usize>,
}

impl Dir {
    fn new(parent: Option<usize>) -> Dir {
        Dir {
            size: 0,
            parent: parent,
            children: HashMap::new(),
        }
    }

    fn total_size(&self, dirs: &[Dir]) -> usize {
        self.size
            + self
                .children
                .values()
                .map(|idx| dirs[*idx].total_size(dirs))
                .sum::<usize>()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut current = 0;
    let mut dir_list: Vec<Dir> = vec![Dir::new(None)];
    for line in input.lines() {
        match line {
            "$ cd /" | "$ ls" => (),
            "$ cd .." => (current = dir_list[current].parent.unwrap()),
            _ if line.starts_with("dir") => {
                let dir_idx = dir_list.len();
                dir_list.push(Dir::new(Some(current)));
                dir_list[current].children.insert(&line[4..], dir_idx);
            }
            _ if line.starts_with("$ cd ") => current = dir_list[current].children[&line[5..]],
            _ => {
                dir_list[current].size += line[..line.find(" ").unwrap()].parse::<usize>().unwrap()
            }
        }
    }

    let part_1 = dir_list
        .iter()
        .map(|d| d.total_size(&dir_list))
        .filter(|&sum| sum <= 100000)
        .sum::<usize>();

    let required_space = dir_list[0].total_size(&dir_list) - 40_000_000;
    let part_2 = dir_list
        .iter()
        .map(|d| d.total_size(&dir_list))
        .filter(|&sum| sum >= required_space)
        .fold(usize::MAX, |mn, s| mn.min(s));

    println!("{:?}", part_1);
    println!("{:?}", part_2);
}
