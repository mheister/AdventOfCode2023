use common::twod::{Grid, Point};
use std::{env, fmt::Debug, fs};

use crate::vis::{
    print_horizontally_reflected_pattern, print_vertically_reflected_pattern,
};
mod vis;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("13/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let patterns = input
        .split("\n\n")
        .map(|s| Grid {
            data: s.lines().flat_map(|l| l.chars()).collect(),
            width: s.lines().next().unwrap_or(&"").len(),
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for pat in &patterns {
        let vert_line = find_vertical_reflection_line(&pat, 0);
        let horz_line = find_horizontal_reflection_line(&pat, 0);
        if vert_line.depth > horz_line.depth {
            print_vertically_reflected_pattern(&pat, &vert_line);
            println!("Adding {}", vert_line.pos);
            println!();
            sum += vert_line.pos;
        } else {
            print_horizontally_reflected_pattern(&pat, &horz_line);
            println!("Adding {}", 100 * horz_line.pos);
            println!();
            sum += 100 * horz_line.pos;
        }
    }
    println!("The sum for part 1 is {sum}");
    println!();

    let mut sum = 0;
    for pat in &patterns {
        let vert_line = find_vertical_reflection_line(&pat, 1);
        let horz_line = find_horizontal_reflection_line(&pat, 1);
        if vert_line.depth > horz_line.depth {
            print_vertically_reflected_pattern(&pat, &vert_line);
            println!("Adding {}", vert_line.pos);
            println!();
            sum += vert_line.pos;
        } else {
            print_horizontally_reflected_pattern(&pat, &horz_line);
            println!("Adding {}", 100 * horz_line.pos);
            println!();
            sum += 100 * horz_line.pos;
        }
    }
    println!("The sum for part 2 is {sum}");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ReflectionLine {
    pos: usize,
    depth: usize,
}

fn find_horizontal_reflection_line(grid: &Grid<char>, smudges: usize) -> ReflectionLine {
    let mut res = ReflectionLine { pos: 0, depth: 0 };
    for pos in 1..grid.height() {
        let mut depth = 0;
        let mut smudges = smudges;
        for d in 0..std::cmp::min(pos, grid.height() - pos) {
            let mut mismatches = (0..grid.width())
                .map(|col| {
                    grid[Point {
                        x: col as i32,
                        y: (pos + d) as i32,
                    }] != grid[Point {
                        x: col as i32,
                        y: (pos - d - 1) as i32,
                    }]
                })
                .filter(|&c| c)
                .count();
            if smudges >= mismatches {
                smudges -= mismatches;
                mismatches = 0;
            }
            if mismatches > 0 {
                break;
            }
            depth += 1;
        }
        if smudges == 0
            && (depth == pos || (depth + pos == grid.height()))
            && depth >= res.depth
        {
            res = ReflectionLine { pos, depth };
        }
    }
    res
}

fn find_vertical_reflection_line(grid: &Grid<char>, smudges: usize) -> ReflectionLine {
    let mut res = ReflectionLine { pos: 0, depth: 0 };
    for pos in 1..grid.width() {
        let mut depth = 0;
        let mut smudges = smudges;
        for d in 0..std::cmp::min(pos, grid.width() - pos) {
            let mut mismatches = (0..grid.height())
                .map(|row| {
                    grid[Point {
                        x: (pos + d) as i32,
                        y: row as i32,
                    }] != grid[Point {
                        x: (pos - d - 1) as i32,
                        y: row as i32,
                    }]
                })
                .filter(|&c| c)
                .count();
            if smudges >= mismatches {
                smudges -= mismatches;
                mismatches = 0;
            }
            if mismatches > 0 {
                break;
            }
            depth += 1;
        }
        if smudges == 0
            && (depth == pos || (depth + pos == grid.width()))
            && depth >= res.depth
        {
            res = ReflectionLine { pos, depth };
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oneline(s: &str) -> Grid<char> {
        Grid {
            data: s.chars().collect(),
            width: s.len(),
        }
    }

    #[test]
    fn find_vertical_reflection_lines() {
        assert_eq!(find_vertical_reflection_line(&oneline(""), 0).depth, 0);
        assert_eq!(find_vertical_reflection_line(&oneline("asdf"), 0).depth, 0);
        assert_eq!(
            find_vertical_reflection_line(&oneline("eert"), 0),
            ReflectionLine { pos: 1, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&oneline("zxx"), 0),
            ReflectionLine { pos: 2, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&oneline("xxxx"), 0),
            ReflectionLine { pos: 2, depth: 2 }
        );
        assert_eq!(
            find_vertical_reflection_line(&oneline("yuipp"), 0),
            ReflectionLine { pos: 4, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&oneline("yui1234pp"), 0),
            ReflectionLine { pos: 8, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&oneline("yui2pp2"), 0),
            ReflectionLine { pos: 5, depth: 2 }
        );
        assert_eq!(
            find_vertical_reflection_line(
                &Grid::<char> {
                    data: "#....#..#\
                       ..##..###\
                       #####.##.\
                       #####.##.\
                       ..##..###\
                       #..4.#.4#"
                        .chars()
                        .collect(),
                    width: 9,
                },
                0
            )
            .depth,
            0
        );
    }

    #[test]
    fn find_horizontal_reflection_lines() {
        assert_eq!(
            find_horizontal_reflection_line(
                &Grid::<char> {
                    data: "#...##..#\
                       #....#..#\
                       ..##..###\
                       #####.##.\
                       #####.##.\
                       ..##..###\
                       #....#..#"
                        .chars()
                        .collect(),
                    width: 9,
                },
                0
            ),
            ReflectionLine { pos: 4, depth: 3 }
        );

        assert_eq!(
            find_horizontal_reflection_line(
                &Grid::<char> {
                    data: "#....#..#\
                       ..##..###\
                       #####.##.\
                       #####.##.\
                       ..##..###\
                       #....#..#"
                        .chars()
                        .collect(),
                    width: 9,
                },
                0
            ),
            ReflectionLine { pos: 3, depth: 3 }
        );
        assert_eq!(
            find_horizontal_reflection_line(
                &Grid::<char> {
                    data: "\
                    ..##.##.##..##..#\
                    ..#.####.#..#.###\
                    #####..#####..###\
                    .#.##..##.#...#.#\
                    .#.##..##.#...#.#\
                    #####..#####..###\
                    ..#.####.#..#.###\
                    ..##.##.##..##..#\
                    ##..####..##.#..#\
                    #...#..#...#..#..\
                    ##..#..##.##.#...\
                    "
                    .chars()
                    .collect(),
                    width: 17,
                },
                0
            ),
            ReflectionLine { pos: 4, depth: 4 }
        );
    }
}
