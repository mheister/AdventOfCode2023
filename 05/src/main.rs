mod data;

use std::{env, fs};

use data::*;
use itertools::Itertools;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("05/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let almanac = parse_almanac(&input);

    let lowest_location = almanac
        .seeds
        .iter()
        .map(|&x| almanac.seed_to_soil_map.lookup(x))
        .map(|x| almanac.soil_to_fertilizer_map.lookup(x))
        .map(|x| almanac.fertilizer_to_water_map.lookup(x))
        .map(|x| almanac.water_to_light_map.lookup(x))
        .map(|x| almanac.light_to_temperature_map.lookup(x))
        .map(|x| almanac.temperature_to_humidity_map.lookup(x))
        .map(|x| almanac.humidity_to_location_map.lookup(x))
        .min()
        .unwrap();

    println!("Lowest location number: {lowest_location}");

    let seed_ranges = almanac
        .seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut seed_range| {
            let &start = seed_range.next().unwrap();
            let &len = seed_range.next().unwrap();
            start..start + len
        })
        .collect::<Vec<_>>();
    let location_ranges = almanac.humidity_to_location_map.lookup_ranges(
        &almanac.temperature_to_humidity_map.lookup_ranges(
            &almanac.light_to_temperature_map.lookup_ranges(
                &almanac.water_to_light_map.lookup_ranges(
                    &almanac.fertilizer_to_water_map.lookup_ranges(
                        &almanac.soil_to_fertilizer_map.lookup_ranges(
                            &almanac.seed_to_soil_map.lookup_ranges(&seed_ranges),
                        ),
                    ),
                ),
            ),
        ),
    );
    let lowest_location_seed_ranges = location_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("Lowest location number considering seed-ranges for part 2: {lowest_location_seed_ranges}");
}

impl AlmanacMap {
    fn lookup(&self, index: usize) -> usize {
        self.lines
            .iter()
            .find(|mapline| {
                (mapline.src_start..mapline.src_start + mapline.len).contains(&index)
            })
            .map(|mapline| index - mapline.src_start + mapline.dst_start)
            .unwrap_or(index.clone())
    }

    fn lookup_range(&self, range: std::ops::Range<usize>) -> Vec<std::ops::Range<usize>> {
        let mut result = vec![];
        let mut bound = range.start;
        while bound < range.end {
            if let Some(mapline) = self.lines.iter().find(|mapline| {
                (mapline.src_start..mapline.src_start + mapline.len).contains(&bound)
            }) {
                let end = std::cmp::min(range.end, mapline.src_start + mapline.len);
                result.push(
                    bound + mapline.dst_start - mapline.src_start
                        ..end + mapline.dst_start - mapline.src_start,
                );
                bound = end;
            } else if let Some(mapline) = self
                .lines
                .iter()
                .find(|mapline| bound < mapline.src_start && mapline.src_start < range.end)
            {
                result.push(bound..mapline.src_start);
                bound = mapline.src_start;
            } else {
                result.push(bound..range.end);
                bound = range.end;
            }
        }
        result
    }

    fn lookup_ranges(
        &self,
        ranges: &Vec<std::ops::Range<usize>>,
    ) -> Vec<std::ops::Range<usize>> {
        ranges.iter().fold(vec![], |mut acc, range| {
            acc.append(&mut self.lookup_range(range.clone()));
            acc
        })
    }
}
