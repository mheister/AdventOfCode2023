#[derive(Debug, Clone)]
pub struct AlmanacMapLine {
    pub dst_start: usize,
    pub src_start: usize,
    pub len: usize,
}

#[derive(Debug, Clone)]
pub struct AlmanacMap {
    pub lines: Vec<AlmanacMapLine>,
}

#[derive(Debug, Clone)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub seed_to_soil_map: AlmanacMap,
    pub soil_to_fertilizer_map: AlmanacMap,
    pub fertilizer_to_water_map: AlmanacMap,
    pub water_to_light_map: AlmanacMap,
    pub light_to_temperature_map: AlmanacMap,
    pub temperature_to_humidity_map: AlmanacMap,
    pub humidity_to_location_map: AlmanacMap,
}

fn parse_almanac_map(input: &str) -> AlmanacMap {
    AlmanacMap {
        lines: input
            .lines()
            .skip(1)
            .map(|line| {
                let mut num_it = line.split_ascii_whitespace().map(str::parse::<usize>);
                AlmanacMapLine {
                    dst_start: num_it.next().expect("missing destination start").unwrap(),
                    src_start: num_it.next().expect("missing source start").unwrap(),
                    len: num_it.next().expect("missing map len").unwrap(),
                }
            })
            .collect(),
    }
}

pub fn parse_almanac(input: &str) -> Almanac {
    let mut section_it = input.split("\n\n");
    let seeds = section_it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let seed_to_soil_map = parse_almanac_map(section_it.next().unwrap());
    let soil_to_fertilizer_map = parse_almanac_map(section_it.next().unwrap());
    let fertilizer_to_water_map = parse_almanac_map(section_it.next().unwrap());
    let water_to_light_map = parse_almanac_map(section_it.next().unwrap());
    let light_to_temperature_map = parse_almanac_map(section_it.next().unwrap());
    let temperature_to_humidity_map = parse_almanac_map(section_it.next().unwrap());
    let humidity_to_location_map = parse_almanac_map(section_it.next().unwrap());
    Almanac {
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    }
}
