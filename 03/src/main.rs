use std::{cmp::min, env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("03/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let sch_nums = get_sch_nums(&input);
    let sch_syms_per_row = get_sch_syms_per_row(&input);
    let part1_sum = calc_part1_sum(&sch_nums, &sch_syms_per_row);
    println!("The sum of part numbers for part 1 is {part1_sum}");

    let gears = sch_syms_per_row
        .iter()
        .cloned()
        .flatten()
        .filter(|sym| sym.sym == '*')
        .collect::<Vec<_>>();
    let part2_sum = calc_part2_sum(&sch_nums, &gears);
    println!("The sum of gear ratios for part 2 is {part2_sum}");
}

fn calc_part2_sum(sch_nums: &Vec<SchematicNumber>, gears: &Vec<SchematicSymbol>) -> u32 {
    let mut sum = 0;
    for gear in gears {
        let row_range = gear.row.saturating_sub(1)..=min(gear.row + 1, sch_nums.len() - 1);
        let adj_nums = sch_nums.iter().filter(|num| {
            row_range.contains(&num.row)
                && (num.col.saturating_sub(1)..=num.col + num.number_str.len())
                    .contains(&gear.col)
        });
        if adj_nums.clone().count() == 2 {
            sum += adj_nums
                .map(|num| num.number_str.parse::<u32>().unwrap())
                .product::<u32>();
        }
    }
    sum
}

fn calc_part1_sum(
    sch_nums: &Vec<SchematicNumber>,
    sch_syms_per_row: &Vec<Vec<SchematicSymbol>>,
) -> u32 {
    sch_nums
        .iter()
        .filter(|&num| {
            let row_range =
                num.row.saturating_sub(1)..=min(num.row + 1, sch_syms_per_row.len() - 1);
            sch_syms_per_row[row_range].iter().any(|sym_row| {
                sym_row.iter().any(|sym| {
                    (num.col.saturating_sub(1)..=num.col + num.number_str.len())
                        .contains(&sym.col)
                })
            })
        })
        .map(|num| num.number_str.parse::<u32>().unwrap())
        .sum()
}

fn get_sch_syms_per_row(input: &str) -> Vec<Vec<SchematicSymbol>> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    '0'..='9' | '.' => None,
                    sym => Some(SchematicSymbol { sym, row, col }),
                })
                .collect()
        })
        .collect()
}

fn get_sch_nums(input: &str) -> Vec<SchematicNumber> {
    let mut nums = Vec::<SchematicNumber>::new();
    for (row, line) in input.lines().enumerate() {
        let mut num = "".to_string();
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
                if !num.is_empty() {
                    let col = col - num.len();
                    nums.push(SchematicNumber {
                        number_str: num,
                        row,
                        col,
                    });
                    num = "".into();
                }
            }
        }
        if !num.is_empty() {
            let col = line.len() - num.len();
            nums.push(SchematicNumber {
                number_str: num,
                row,
                col,
            });
        }
    }
    nums
}

#[derive(Clone)]
struct SchematicNumber {
    number_str: String,
    row: usize,
    col: usize,
}

#[derive(Clone)]
struct SchematicSymbol {
    sym: char,
    row: usize,
    col: usize,
}
