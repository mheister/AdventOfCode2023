use std::{collections::HashMap, env, fs};

use itertools::{FoldWhile, Itertools};

type Node = [u8; 3];

fn node_from_str(string: &str) -> Node {
    string.as_bytes().try_into().unwrap()
}

#[derive(Debug)]
struct CamelMap {
    instruction: String,
    mapping: HashMap<Node, (Node, Node)>,
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
            .map(|caps| {
                (
                    node_from_str(&caps[1]),
                    (node_from_str(&caps[2]), node_from_str(&caps[3])),
                )
            })
            .collect::<HashMap<_, _>>();
        Ok(CamelMap {
            instruction: instruction.to_owned(),
            mapping,
        })
    }
}

#[derive(Debug, Clone)]
struct Periodicity {
    start_cycle: usize,
    period: usize,
    start_node: Node,
}

fn get_periodicity(map: &CamelMap, mut start: Node) -> Periodicity {
    let mut start_of_cycle_nodes: HashMap<Node, usize> = Default::default();
    for (cycle, instructions) in std::iter::repeat(map.instruction.as_bytes()).enumerate()
    {
        if let Some(start_cycle) = start_of_cycle_nodes.get(&start) {
            return Periodicity {
                start_cycle: *start_cycle,
                period: cycle - start_cycle,
                start_node: start,
            };
        }
        start_of_cycle_nodes.insert(start, cycle);
        for inst in instructions {
            let branches = &map.mapping[&start];
            start = match inst {
                b'L' => branches.0,
                b'R' => branches.1,
                _ => panic!(),
            }
        }
    }
    panic!();
}
fn get_z_node_indices(periodicity: Periodicity, map: &CamelMap) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    let mut node = periodicity.start_node;
    std::iter::repeat(map.instruction.as_bytes())
        .take(periodicity.period)
        .flatten()
        .enumerate()
        .for_each(|(idx, inst)| {
            if node[2] == b'Z' {
                res.push(idx.try_into().unwrap());
            }
            let branches = &map.mapping[&node];
            node = match inst {
                b'L' => branches.0,
                b'R' => branches.1,
                _ => panic!(),
            }
        });
    res
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("08/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let map: CamelMap = input.as_str().try_into().unwrap();

    let p1_steps = map
        .instruction
        .chars()
        .cycle()
        .fold_while((['A' as u8; 3], 0), |acc, inst| {
            if !map.mapping.contains_key(&acc.0) {
                return FoldWhile::Done((['Z' as u8; 3], 0));
            }
            let branches = &map.mapping[&acc.0];
            let next = match inst {
                'L' => branches.0,
                'R' => branches.1,
                _ => panic!(),
            };
            if next == ['Z' as u8; 3] {
                FoldWhile::Done((next, acc.1 + 1))
            } else {
                FoldWhile::Continue((next, acc.1 + 1))
            }
        })
        .into_inner()
        .1;

    println!("Number of steps for part 1: {p1_steps}");

    let p2_starting_pos = map
        .mapping
        .keys()
        .filter(|pos| pos[2] == 'A' as u8)
        .collect::<Vec<_>>();
    dbg!(&p2_starting_pos);

    let periodicities = p2_starting_pos
        .iter()
        .map(|node| get_periodicity(&map, **node))
        .map(|periodicity| (periodicity.clone(), get_z_node_indices(periodicity, &map)))
        .map(|(periodicity, z_indices)| {
            assert_eq!(z_indices.len(), 1); // always one with this input
            let first_z_step = periodicity.start_cycle * map.instruction.len()
                + *z_indices.first().unwrap() as usize;
            let z_step_period = periodicity.period * map.instruction.len();
            assert_eq!(first_z_step, z_step_period); // input seems designed so
            first_z_step
        })
        .collect::<Vec<_>>();
    dbg!(&periodicities);

    let p2_steps = periodicities
        .iter()
        .fold(1, |acc, &p| num::integer::lcm(acc, p));

    println!("Number of steps for part 2: {p2_steps}");
}
