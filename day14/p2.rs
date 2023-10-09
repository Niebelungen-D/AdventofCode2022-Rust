#![feature(iter_from_generator)]
#![feature(generators)]
#![feature(drain_filter)]
use ::std::fmt;
use std::vec;

const SPAWN_POINT: Point = Point { x: 500, y: 0 };

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, derive_more::Add, derive_more::AddAssign, derive_more::Sub,
)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(s: &str) -> Point {
        let mut token = s.split(",");
        let x = token.next().unwrap().parse::<i32>().unwrap();
        let y = token.next().unwrap().parse::<i32>().unwrap();
        Point { x, y }
    }

    fn signum(&self) -> Point {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

#[derive(Debug)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn parse(s: &str) -> Polyline {
        let points = s.split(" -> ").map(|x| Point::parse(x)).collect();
        Polyline { points }
    }

    fn path_points(&self) -> impl Iterator<Item = Point> + '_ {
        std::iter::from_generator(|| {
            let mut points = self.points.iter().copied();
            let Some(mut a) = points.next() else { return };
            yield a; // here the first point is yielded

            loop {
                let Some(b) = points.next() else { return };
                let delta = (b - a).signum();
                assert!((delta.x == 0) ^ (delta.y == 0));

                loop {
                    a += delta;
                    yield a;
                    if a == b {
                        break;
                    }
                }
            }
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Air,
    Rock,
    Sand,
}

// Note: X+ goes right, Y+ goes down
struct Grid {
    settled: usize,
    grains: Vec<Point>,
    origin: Point,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut polylines: Vec<_> = input.lines().map(Polyline::parse).collect();

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        // sand falls from `(500,0)`
        let sand_spawn = Point { x: 500, y: 0 };

        for point in polylines
            .iter()
            .flat_map(|p| p.points.iter())
            .chain(std::iter::once(&sand_spawn))
        {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        dbg!(min_x, max_x);
        dbg!(min_y, max_y);
        max_y += 2; // we need to add 1 for the floor, and 1 for the spawn point 
        min_x -= max_y - min_y ; 
        max_x += max_y - min_y + 2;
        let origin = Point { x: min_x, y: min_y };
        let height: usize = (max_y - min_y + 1).try_into().unwrap();
        let width: usize = (max_x - min_x + 1).try_into().unwrap();
        dbg!(origin, width, height);
        let mut grid = Self {
            origin,
            width,
            height,
            cells: vec![Cell::Air; width * height],
            grains: vec![SPAWN_POINT],
            settled: 0,
        };

        polylines.push(Polyline {
            points: vec![Point { x: min_x, y: max_y }, Point { x: max_x, y: max_y }],
        });
        for point in polylines.iter().flat_map(|p| p.path_points()) {
            // println!("{:?}", point);
            *grid.cell_mut(point).unwrap() = Cell::Rock;
        }
        grid
    }

    fn cell_index(&self, point: Point) -> Option<usize> {
        let Point { x, y } = point - self.origin;
        // negative coords after offsetting = outside of grid
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn cell(&self, point: Point) -> Option<&Cell> {
        Some(&self.cells[self.cell_index(point)?])
    }

    fn cell_mut(&mut self, point: Point) -> Option<&mut Cell> {
        // borrow checker won't let us do that inline 🙃
        let cell_index = self.cell_index(point)?;
        Some(&mut self.cells[cell_index])
    }

    pub fn step(&mut self) {
        if matches!(self.cell(Point { x: 500, y: 0 }).unwrap(), Cell::Sand) {
            // don't step, we're done
            return;
        }

        // this is _relatively_ cheap. an empty vec doesn't actually make any
        // heap allocations iirc, and we're re-using the storage of `grains`
        // every turn
        let mut grains = std::mem::take(&mut self.grains);
        let _ = grains
            .drain_filter(|grain| {
                let straight_down = *grain + Point { x: 0, y: 1 };
                let down_left = *grain + Point { x: -1, y: 1 };
                let down_right = *grain + Point { x: 1, y: 1 };
                let options = [straight_down, down_left, down_right];

                // Can we move?
                if let Some(pos) = options
                    .into_iter()
                    .find(|pos| matches!(self.cell(*pos), Some(Cell::Air)))
                {
                    *grain = pos;
                    return false; // keep it
                }

                // If not, are we moving off-screen?
                if options.into_iter().any(|pos| self.cell(pos).is_none()) {
                    return true; // remove it
                }

                // If not, then we've settled
                self.settled += 1;
                *self.cell_mut(*grain).unwrap() = Cell::Sand;
                true // remove it
            })
            .count();
        self.grains = grains;
        self.grains.push(SPAWN_POINT);
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point {
                    x: x as _,
                    y: y as _,
                } + self.origin;
                let cell = self.cell(point).unwrap();
                let c = match cell {
                    Cell::Air => '.',
                    Cell::Rock => '#',
                    Cell::Sand => 'o',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut grid = Grid::parse(include_str!("../input"));
    // println!("{:?}", grid);
    let mut prev = grid.settled;
    for _ in 0..100000 {
        for _ in 0..1000{
            grid.step();
        }
        if grid.settled == prev {
            break;
        }
        prev = grid.settled;
    } 
    println!("{:?}", grid.settled);
}
