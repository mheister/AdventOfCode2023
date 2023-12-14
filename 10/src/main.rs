use common::twod::{Direction as Dir, Grid, Point, PointNeighbours};
use std::{collections::HashMap, env, fs, ops::Index};

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
    let opposites = HashMap::from([
        (Dir::E, Dir::W),
        (Dir::W, Dir::E),
        (Dir::N, Dir::S),
        (Dir::S, Dir::N),
    ]);
    // crawl all around once to gen len
    let mut p = map.start;
    let mut origin_dir = Dir::S;
    let mut path = vec![];
    while path.len() < 1_000_000 {
        (origin_dir, p) = map
            .neighbours(p)
            .filter(|(dir, _)| opposites[dir] != origin_dir)
            .filter(|(dir, _)| pipe_ends[&map[p]].contains(dir))
            .find(|(dir, neighbour)| pipe_ends[&map[*neighbour]].contains(&opposites[dir]))
            .expect(format!("Stuck at {p}").as_str());
        path.push(p);
        if map[p] == b'S' {
            break;
        }
    }
    println!(
        "Len is {}, halfway point is {}, {} steps away from start",
        path.len(),
        path[path.len() / 2 - 1],
        path.len() / 2
    );
}

struct TubeMap {
    grid: Grid<u8>,
    start: Point,
}

impl From<&str> for TubeMap {
    fn from(value: &str) -> Self {
        let width = value.lines().next().map(|l| l.len()).unwrap_or(0);
        let map = value
            .as_bytes()
            .iter()
            .cloned()
            .filter(|&c| c != b'\n')
            .collect::<Vec<_>>();
        let start_idx_in_raw = map
            .iter()
            .position(|&c| c == b'S')
            .expect("Could not find start");
        TubeMap {
            grid: Grid { data: map, width },
            start: Point {
                x: (start_idx_in_raw as usize % width).try_into().unwrap(),
                y: (start_idx_in_raw as usize / width).try_into().unwrap(),
            },
        }
    }
}

impl Index<Point> for TubeMap {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index]
    }
}

impl TubeMap {
    fn neighbours(&self, p: Point) -> PointNeighbours {
        p.neighbours(self.grid.width() as i32, self.grid.height() as i32)
    }
}
