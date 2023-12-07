use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("06/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t_str| str::parse::<usize>(t_str).unwrap());
    let records = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t_str| str::parse::<usize>(t_str).unwrap());
    let races = times
        .zip(records)
        .map(|(t, d)| Race {
            time_ms: t,
            record_mm: d,
        })
        .collect::<Vec<_>>();
    let p1_product = races
        .iter()
        .cloned()
        .map(count_winning_options)
        .product::<usize>();
    println!("Product of margins for part 1: {p1_product}");

    let p2_time = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let p2_record = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    // brute forcing a quadratic equation but ok
    let p2_options = count_winning_options(Race {
        time_ms: p2_time,
        record_mm: p2_record,
    });
    println!("Margin for part 2: {p2_options}");
}

#[derive(Clone, Copy)]
struct Race {
    time_ms: usize,
    record_mm: usize,
}

fn count_winning_options(race: Race) -> usize {
    (0..race.time_ms)
        .filter(|t_push| {
            let dist = t_push * (race.time_ms - t_push);
            dist > race.record_mm
        })
        .count()
        .try_into()
        .unwrap()
}
