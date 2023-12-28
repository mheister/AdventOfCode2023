use crate::ReflectionLine;
use common::twod::Grid;
use itertools::Itertools;

pub fn print_horizontally_reflected_pattern(pat: &Grid<char>, line: &ReflectionLine) {
    let lines = pat.data.iter().chunks(pat.width);
    for (idx, row) in lines.into_iter().enumerate() {
        let designator = match idx {
            idx if idx == (line.pos - 1) => 'v',
            idx if idx == line.pos => '^',
            idx if idx == line.pos.saturating_sub(line.depth) => '-',
            idx if idx == line.pos + line.depth - 1 => '-',
            _ => ' ',
        };
        println!("{:2}{designator}{}", idx + 1, row.collect::<String>());
    }
}

pub fn print_vertically_reflected_pattern(pat: &Grid<char>, line: &ReflectionLine) {
    println!("{}", "123456789".chars().cycle().take(pat.width).collect::<String>());
    println!("{}><", std::iter::repeat(' ').take(line.pos - 1).collect::<String>());
    let lines = pat.data.iter().chunks(pat.width);
    for row in lines.into_iter() {
        println!("{}", row.collect::<String>());
    }
    println!("{}><", std::iter::repeat(' ').take(line.pos - 1).collect::<String>());
}
