use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("11/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let width = input.lines().next().map(|l| l.len()).unwrap_or(0);
    let mut xp_cols: Vec<usize> = (0..width).collect();
    let mut xp_rows = vec![];
    let mut galaxies = vec![];
    for (row, line) in input.lines().enumerate() {
        if line.chars().all(|c| c == '.') {
            xp_rows.push(row);
            continue;
        }
        galaxies.extend(
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(col, _)| (row, col)),
        );
        xp_cols.retain(|&col| line.chars().nth(col).unwrap() == '.');
    }
    let sum_of_distances = calc_sum_of_distances(&galaxies, &xp_rows, &xp_cols, 2);
    println!("The sum of distances for part 1 is {sum_of_distances}");
    let sum_of_distances = calc_sum_of_distances(&galaxies, &xp_rows, &xp_cols, 10);
    println!(
        "The sum of distances considering an expansion factor of 10 is {sum_of_distances}"
    );
    let sum_of_distances = calc_sum_of_distances(&galaxies, &xp_rows, &xp_cols, 1_000_000);
    println!(
        "The sum of distances considering an expansion factor of 1M is {sum_of_distances}"
    );
}

fn calc_sum_of_distances(
    galaxies: &Vec<(usize, usize)>,
    xp_rows: &Vec<usize>,
    xp_cols: &Vec<usize>,
    expansion_factor: usize,
) -> usize {
    let mut sum_of_distances = 0;
    for (a, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies.iter().skip(a + 1) {
            let vert_expansion = xp_rows
                .iter()
                .filter(|&&row| {
                    std::cmp::min(galaxy_a.0, galaxy_b.0) < row
                        && row < std::cmp::max(galaxy_a.0, galaxy_b.0)
                })
                .count();
            let horz_expansion = xp_cols
                .iter()
                .filter(|&&col| {
                    std::cmp::min(galaxy_a.1, galaxy_b.1) < col
                        && col < std::cmp::max(galaxy_a.1, galaxy_b.1)
                })
                .count();
            sum_of_distances += std::cmp::max(galaxy_a.1, galaxy_b.1)
                - std::cmp::min(galaxy_a.1, galaxy_b.1)
                + horz_expansion * (expansion_factor - 1)
                + std::cmp::max(galaxy_a.0, galaxy_b.0)
                - std::cmp::min(galaxy_a.0, galaxy_b.0)
                + vert_expansion * (expansion_factor - 1);
        }
    }
    sum_of_distances
}
