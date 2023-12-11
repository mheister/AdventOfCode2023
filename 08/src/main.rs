use std::{collections::HashMap, env, fs};

use itertools::{FoldWhile, Itertools};

#[derive(Debug)]
struct CamelMap {
    instruction: String,
    mapping: HashMap<String, (String, String)>,
}

impl TryFrom<&str> for CamelMap {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (instruction, mapping_str) = value
            .split_once("\n\n")
            .ok_or("expecting two sections".to_owned())?;
        let mapping_re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let mapping = mapping_re
            .captures_iter(mapping_str)
            .map(|caps| (caps[1].to_owned(), (caps[2].to_owned(), caps[3].to_owned())))
            .collect::<HashMap<_, _>>();
        Ok(CamelMap {
            instruction: instruction.to_owned(),
            mapping,
        })
    }
}

fn main() {
    let input_file_path = env::args()
        .nth(1)
        .unwrap_or("08/example_input_1.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let map: CamelMap = input.as_str().try_into().unwrap();

    let p1_steps = map
        .instruction
        .chars()
        .cycle()
        .fold_while(("AAA", 0), |acc, inst| {
            if !map.mapping.contains_key(acc.0) {
                return FoldWhile::Done(("ZZZ", 0));
            }
            let branches = &map.mapping[acc.0];
            let next = match inst {
                'L' => &branches.0,
                'R' => &branches.1,
                _ => panic!(),
            };
            if next == "ZZZ" {
                FoldWhile::Done((&next, acc.1 + 1))
            } else {
                FoldWhile::Continue((&next, acc.1 + 1))
            }
        })
        .into_inner()
        .1;

    println!("Number of steps for part 1: {p1_steps}");

    let p2_starting_pos = map
        .mapping
        .keys()
        .filter(|pos| pos.chars().last().unwrap() == 'A')
        .collect::<Vec<_>>();
    dbg!(&p2_starting_pos);

    let p2_steps = map
        .instruction
        .chars()
        .cycle()
        .fold_while((p2_starting_pos, 0), |mut acc, inst| {
            for pos in &mut acc.0 {
                let branches = &map.mapping[*pos];
                *pos = match inst {
                    'L' => &branches.0,
                    'R' => &branches.1,
                    _ => panic!(),
                };
            }
            acc.1 += 1;
            if acc.0.iter().all(|pos| pos.chars().last().unwrap() == 'Z') {
                FoldWhile::Done(acc)
            } else {
                FoldWhile::Continue(acc)
            }
        })
        .into_inner()
        .1;

    println!("Number of steps for part 2: {p2_steps}");
}
