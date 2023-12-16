mod tubemap;

use common::twod::{Direction as Dir, Point};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

use crate::tubemap::TubeMap;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("10/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let map = TubeMap::from(input.as_str());
    let pipe_ends = HashMap::<_, _>::from([
        (b'|', vec![Dir::N, Dir::S]),
        (b'-', vec![Dir::W, Dir::E]),
        (b'L', vec![Dir::N, Dir::E]),
        (b'J', vec![Dir::W, Dir::N]),
        (b'7', vec![Dir::W, Dir::S]),
        (b'F', vec![Dir::E, Dir::S]),
        (b'.', vec![]),
        (b'S', vec![Dir::E, Dir::S, Dir::W, Dir::N]),
    ]);

    // crawl around animal's path to get len
    let animal_path = try_crawl_path(&map, map.start, &pipe_ends)
        .expect("Failed to crawl the animal's path");
    println!(
        "Len is {}, halfway point is {}, {} steps away from start",
        animal_path.len(),
        animal_path[animal_path.len() / 2 - 1],
        animal_path.len() / 2
    );

    // find the enclosed tiles for part 2
    let mut map = map.clone();
    let floodfill = floodfill_path(&animal_path, &map);

    // it turns out finding inner paths is not the point, everything in the floodfill
    // counts as enclosed
    //
    // for p in &floodfill {
    //     match map[*p] {
    //         b'.' | b'X' => continue,
    //         _ => (),
    //     }
    //     if let Some(path) = try_crawl_path(&map, *p, &pipe_ends) {
    //         for inner_path_content in floodfill_path(&path, &map) {
    //             map[inner_path_content] = b'X';
    //         }
    //         for inner_path_seg in path {
    //             // to prevent us from crawling this path again
    //             map[inner_path_seg] = b'X';
    //         }
    //     }
    // }
    // let n_tiles = floodfill.iter().filter(|&&p| map[p] != b'X').count();

    let n_tiles = floodfill.len();
    println!("There are {n_tiles} tiles enclosed by the loop");

    // print the map as we see it
    (0..map.grid.width())
        .cartesian_product(0..map.grid.height())
        .map(|(x, y)| Point {
            x: x as i32,
            y: y as i32,
        })
        .for_each(|p| {
            if animal_path.contains(&p) {
                //
            } else if floodfill.contains(&p) {
                map[p] = b'.';
            } else {
                map[p] = b'x';
            }
        });
    println!("map:\n{}", map.to_string());
}

fn try_crawl_path(
    map: &TubeMap,
    start: Point,
    pipe_ends: &HashMap<u8, Vec<Dir>>,
) -> Option<Vec<Point>> {
    let mut origin_dir = Dir::S;
    let mut path = vec![];
    let mut p = start;
    while path.len() < 1_000_000 {
        (origin_dir, p) = map
            .neighbours(p)
            .filter(|(dir, _)| dir.opposite() != origin_dir)
            .filter(|(dir, _)| pipe_ends[&map[p]].contains(dir))
            .find(|(dir, neighbour)| {
                pipe_ends[&map[*neighbour]].contains(&dir.opposite())
            })?;
        path.push(p);
        if p == start {
            break;
        }
    }
    Some(path)
}

fn ensure_clockwise_path(path: &Vec<Point>) -> Vec<Point> {
    if path.is_empty() {
        return vec![];
    }
    let miny = path.iter().map(|p| p.y).min().unwrap();
    let (topleft_pos, topleft) = path
        .iter()
        .enumerate()
        .filter(|(_, p)| p.y == miny)
        .min_by(|a, b| a.1.x.cmp(&b.1.x))
        .unwrap();
    let next = path
        .iter()
        .nth(topleft_pos + 1)
        .unwrap_or(path.first().unwrap());
    if next.x - topleft.x == 1 {
        path.clone()
    } else {
        path.iter().cloned().rev().collect()
    }
}

fn floodfill_path(path: &Vec<Point>, map: &TubeMap) -> HashSet<Point> {
    let animal_path = ensure_clockwise_path(&path);
    let mut floodfill = HashSet::new();
    for (a, b) in animal_path.iter().zip(animal_path.iter().cycle().skip(1)) {
        let dir = (b.x - a.x, b.y - a.y);
        let dir_rot90 = (-1 * dir.1, dir.0);
        for s in [a, b] {
            let floodfill_from_here = map.grid.get_floodfill_region(
                Point {
                    x: s.x + dir_rot90.0,
                    y: s.y + dir_rot90.1,
                },
                |p| !animal_path.contains(&p),
            );
            floodfill.extend(floodfill_from_here);
        }
    }
    floodfill
}
