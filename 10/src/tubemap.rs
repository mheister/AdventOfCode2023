use common::twod::Grid;
use common::twod::Point;
use common::twod::PointNeighbours;

use std::ops::Index;
use std::ops::IndexMut;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct TubeMap {
    pub grid: Grid<u8>,
    pub start: Point,
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

impl From<&TubeMap> for String {
    fn from(value: &TubeMap) -> Self {
        value
            .grid
            .data
            .iter()
            .map(|&c| c as char)
            .map(|c| match c {
                '|' => '│',
                '-' => '─',
                'L' => '└',
                'J' => '┘',
                '7' => '┐',
                'F' => '┌',
                c => c,
            })
            .chunks(value.grid.width())
            .into_iter()
            .map(|chunk| chunk.chain("\n".chars()))
            .flatten()
            .collect()
    }
}

impl std::fmt::Display for TubeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(self).as_str())
    }
}

impl Index<Point> for TubeMap {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<Point> for TubeMap {
    fn index_mut(&mut self, index: Point) -> &mut u8 {
        &mut self.grid[index]
    }
}

impl TubeMap {
    pub fn neighbours(&self, p: Point) -> PointNeighbours {
        p.neighbours(self.grid.width() as i32, self.grid.height() as i32)
    }
}
