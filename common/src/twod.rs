use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn opposite(self) -> Self {
        let opposites = HashMap::from([
            (Direction::E, Direction::W),
            (Direction::W, Direction::E),
            (Direction::N, Direction::S),
            (Direction::S, Direction::N),
        ]);
        opposites[&self]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    /// x coordinate, alias column
    pub x: i32,
    /// y coordinate, alias row
    pub y: i32,
}

impl Point {
    pub fn neighbours(&self, width: i32, height: i32) -> PointNeighbours {
        PointNeighbours {
            x: self.x,
            y: self.y,
            x_lim: width,
            y_lim: height,
            i: 0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

pub struct PointNeighbours {
    x: i32,
    y: i32,
    x_lim: i32,
    y_lim: i32,
    i: u8,
}

impl Iterator for PointNeighbours {
    type Item = (Direction, Point);
    fn next(&mut self) -> Option<Self::Item> {
        use Direction as D;
        const OFFSETS: [(Direction, i32, i32); 4] =
            [(D::E, 1, 0), (D::S, 0, 1), (D::W, -1, 0), (D::N, 0, -1)];
        let mut dir;
        let mut point;
        loop {
            if self.i as usize >= OFFSETS.len() {
                return None;
            }
            let off = OFFSETS[self.i as usize];
            self.i += 1;
            dir = off.0;
            point = Point {
                x: self.x + off.1,
                y: self.y + off.2,
            };
            if point.x >= 0 && point.y >= 0 && point.x < self.x_lim && point.y < self.y_lim
            {
                break;
            }
        }
        Some((dir, point))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, p: Point) -> &Self::Output {
        &self.data[p.x as usize + self.width * p.y as usize]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        &mut self.data[p.x as usize + self.width * p.y as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RowIdx {
    pub idx: usize,
}

impl<T> std::ops::Index<RowIdx> for Grid<T> {
    type Output = [T];
    fn index(&self, r: RowIdx) -> &Self::Output {
        &self.data[r.idx * self.width..(r.idx + 1) * self.width]
    }
}

impl<T> std::ops::IndexMut<RowIdx> for Grid<T> {
    fn index_mut(&mut self, r: RowIdx) -> &mut [T] {
        &mut self.data[r.idx * self.width..(r.idx + 1) * self.width]
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn ensure_height(&mut self, min_height: usize, fill: T) {
        let min_size = min_height * self.width;
        if self.data.len() < min_size {
            self.data.resize(min_size, fill);
        }
    }

    pub fn fill_path(&mut self, path: &[Point], item: T) {
        for segment in path.windows(2) {
            self.fill_line(*segment.get(0).unwrap(), *segment.get(1).unwrap(), item);
        }
    }

    pub fn fill_line(&mut self, p1: Point, p2: Point, item: T) {
        let mut p = p1;
        while p != p2 {
            self[p] = item;
            let (step_x, step_y) = ((p2.x - p1.x).signum(), (p2.y - p1.y).signum());
            p = Point {
                x: p.x + step_x,
                y: p.y + step_y,
            }
        }
        self[p2] = item;
    }

    pub fn get_floodfill_region<P>(&self, start: Point, predicate: P) -> HashSet<Point>
    where
        P: Fn(Point) -> bool,
    {
        if !predicate(start) {
            return HashSet::new();
        }
        let mut to_visit = vec![start];
        let mut visited = HashSet::new();
        while !to_visit.is_empty() {
            let p = to_visit.pop().unwrap();
            visited.insert(p);
            for (_, n) in p.neighbours(self.width as i32, self.height() as i32) {
                if !visited.contains(&n) && predicate(n) {
                    to_visit.push(n);
                }
            }
        }
        visited
    }

    pub fn mirror_rows(&mut self) {
        let mut buf: Vec<T>;
        for row in 0..self.height() {
            buf = self[RowIdx { idx: row }].iter().cloned().collect();
            for col in 0..self.width {
                self[RowIdx { idx: row }][col] = buf[self.width - col];
            }
        }
    }

    pub fn mirror_columns(&mut self) {
        self.data = self.data.iter().rev().cloned().collect();
        self.mirror_rows();
    }
}
