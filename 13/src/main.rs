use common::twod::{Grid, Point, RowIdx};
use std::{env, fmt::Debug, fs};

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
    for pat in patterns {
        let vert_line = (0..pat.height())
            .map(|row| find_vertical_reflection_line(&pat[RowIdx { idx: row }]))
            .min_by(|a, b| a.depth.cmp(&b.depth))
            .unwrap();
        let horz_line = (0..pat.width())
            .map(|col| find_horizontal_reflection_line(&pat, col))
            .min_by(|a, b| a.depth.cmp(&b.depth))
            .unwrap();
        if vert_line.depth > horz_line.depth {
            dbg!(vert_line);
            sum += vert_line.pos;
        } else {
            dbg!(horz_line);
            sum += 100 * horz_line.pos;
        }
    }
    print!("The sum for part 1 is {sum}");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ReflectionLine {
    pos: usize,
    depth: usize,
}

fn find_horizontal_reflection_line(grid: &Grid<char>, col: usize) -> ReflectionLine {
    let mut res = ReflectionLine { pos: 0, depth: 0 };
    for pos in 1..grid.height() {
        let mut depth = 0;
        for d in 0..std::cmp::min(pos, grid.height() - pos) {
            if grid[Point {
                x: col as i32,
                y: (pos + d) as i32,
            }] != grid[Point {
                x: col as i32,
                y: (pos - d - 1) as i32,
            }] {
                break;
            }
            depth += 1;
        }
        if depth > res.depth {
            res = ReflectionLine { pos, depth };
        }
    }
    res
}

fn find_vertical_reflection_line(row: &[char]) -> ReflectionLine {
    let mut res = ReflectionLine { pos: 0, depth: 0 };
    for pos in 1..row.len() {
        let depth = row
            .iter()
            .clone()
            .skip(pos)
            .zip(row.iter().rev().skip(row.len() - pos))
            .map_while(|(a, b)| if a == b { Some(()) } else { None })
            .count();
        if depth > res.depth {
            res = ReflectionLine { pos, depth };
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn charvec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn find_vertical_reflection_lines() {
        assert_eq!(find_vertical_reflection_line(&charvec("")).depth, 0);
        assert_eq!(find_vertical_reflection_line(&charvec("asdf")).depth, 0);
        assert_eq!(
            find_vertical_reflection_line(&charvec("eert")),
            ReflectionLine { pos: 1, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&charvec("zxxc")),
            ReflectionLine { pos: 2, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&charvec("xxxx")),
            ReflectionLine { pos: 2, depth: 2 }
        );
        assert_eq!(
            find_vertical_reflection_line(&charvec("yuipp")),
            ReflectionLine { pos: 4, depth: 1 }
        );
        assert_eq!(
            find_vertical_reflection_line(&charvec("yui2pp2")),
            ReflectionLine { pos: 5, depth: 2 }
        );
    }

    #[test]
    fn find_horizontal_reflection_lines() {
        for col in 0..9 {
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
                    col,
                ),
                ReflectionLine { pos: 4, depth: 3 }
            );
        }
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
                0,
            ),
            ReflectionLine { pos: 3, depth: 3 }
        );
    }
}
