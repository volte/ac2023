use std::collections::HashSet;

use nalgebra_glm::IVec2;

use crate::aoc::prelude::*;

pub struct SolutionPart1;
pub struct SolutionPart2;

pub const SOLUTIONS: Solutions = (&SolutionPart1, &SolutionPart2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Winding {
    CW,
    CCW,
}

impl Dir {
    fn delta(&self) -> IVec2 {
        use Dir::*;
        match self {
            North => IVec2::new(0, -1),
            South => IVec2::new(0, 1),
            East => IVec2::new(1, 0),
            West => IVec2::new(-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        use Dir::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    fn inside_dir(&self, winding: Winding) -> Self {
        use Dir::*;
        let dir = match self {
            North => East,
            South => West,
            East => South,
            West => North,
        };
        match winding {
            Winding::CW => dir,
            Winding::CCW => dir.opposite(),
        }
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        use Tile::*;
        match c {
            '.' => Ground,
            'S' => Start,
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => panic!("Invalid tile: {}", c),
        }
    }

    fn get_dirs(&self) -> (Dir, Dir) {
        use Dir::*;
        use Tile::*;

        match self {
            NorthSouth => (North, South),
            EastWest => (East, West),
            NorthEast => (North, East),
            NorthWest => (North, West),
            SouthWest => (South, West),
            SouthEast => (South, East),
            _ => panic!("Invalid tile: {:?}", self),
        }
    }

    fn connects_to(&self, dir: Dir) -> bool {
        self.traverse(dir.opposite()).is_some()
    }

    fn traverse(&self, dir: Dir) -> Option<Dir> {
        use Dir::*;
        use Tile::*;

        match (self, dir) {
            (NorthSouth, North) => Some(North),
            (NorthSouth, South) => Some(South),
            (EastWest, East) => Some(East),
            (EastWest, West) => Some(West),
            (NorthEast, South) => Some(East),
            (NorthEast, West) => Some(North),
            (NorthWest, South) => Some(West),
            (NorthWest, East) => Some(North),
            (SouthWest, North) => Some(West),
            (SouthWest, East) => Some(South),
            (SouthEast, North) => Some(East),
            (SouthEast, West) => Some(South),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    tiles: Vec<Tile>,
    width: i32,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: vec![Tile::Ground; (width * height) as usize],
            width,
        }
    }

    fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            width = line.len() as i32;
            for c in line.chars() {
                tiles.push(Tile::from_char(c));
            }
        }
        Self { tiles, width }
    }

    fn height(&self) -> i32 {
        self.tiles.len() as i32 / self.width
    }

    fn get(&self, coord: IVec2) -> Tile {
        self.tiles[(coord.y * self.width + coord.x) as usize]
    }

    fn set(&mut self, coord: IVec2, tile: Tile) {
        self.tiles[(coord.y * self.width + coord.x) as usize] = tile;
    }

    fn start(&self) -> IVec2 {
        let pos = self
            .tiles
            .iter()
            .position(|&t| t == Tile::Start)
            .expect("No start found");
        let x = pos as i32 % self.width;
        let y = pos as i32 / self.width;
        IVec2::new(x as i32, y as i32)
    }

    fn infer(&self, coord: IVec2) -> Tile {
        use Dir::*;
        use Tile::*;

        let north = self.get(coord + North.delta()).connects_to(South);
        let east = self.get(coord + East.delta()).connects_to(West);
        let south = self.get(coord + South.delta()).connects_to(North);
        let west = self.get(coord + West.delta()).connects_to(East);

        match (north, east, south, west) {
            (true, false, true, false) => NorthSouth,
            (false, true, false, true) => EastWest,
            (true, true, false, false) => NorthEast,
            (true, false, false, true) => NorthWest,
            (false, false, true, true) => SouthWest,
            (false, true, true, false) => SouthEast,
            _ => panic!("{} {} {} {}", north, east, south, west),
        }
    }
}

#[derive(Debug, Clone)]
struct Cursor<'g> {
    grid: &'g Grid,
    pos: IVec2,
    dir: Dir,
}

impl<'g> Cursor<'g> {
    fn new(grid: &'g Grid, pos: IVec2, dir: Dir) -> Self {
        Self { grid, pos, dir }
    }

    fn next(&mut self) -> IVec2 {
        self.pos += self.dir.delta();
        self.dir = self.grid.get(self.pos).traverse(self.dir).unwrap();
        self.pos
    }
}

fn flood_fill(grid: &Grid, coord: IVec2, interior: &mut HashSet<IVec2>) {
    use Dir::*;
    use Tile::*;

    let mut queue = vec![coord];

    while !queue.is_empty() {
        let coord = queue.pop().unwrap();
        match grid.get(coord) {
            Ground => {
                interior.insert(coord);
                for dir in [North, East, South, West].iter() {
                    let next = coord + dir.delta();
                    if !interior.contains(&next) {
                        queue.push(next);
                    }
                }
            }
            _ => continue,
        }
    }
}

impl Aoc for SolutionPart1 {
    fn solve(&self, input: &str) -> String {
        let mut grid = Grid::parse(input);

        let start_coord = grid.start();
        let start_tile = grid.infer(start_coord);
        let start_dirs = start_tile.get_dirs();
        grid.set(start_coord, start_tile);

        let mut cursor1 = Cursor::new(&grid, start_coord, start_dirs.0);
        let mut cursor2 = Cursor::new(&grid, start_coord, start_dirs.1);

        let mut steps = 0;
        loop {
            let (_, coord2) = (cursor1.pos, cursor2.pos);
            let (next1, next2) = (cursor1.next(), cursor2.next());

            steps += 1;

            if next1 == next2 || next1 == coord2 {
                break;
            }
        }

        steps.to_string()
    }
}

impl Aoc for SolutionPart2 {
    fn solve(&self, input: &str) -> String {
        let mut grid = Grid::parse(input);
        let mut clean_grid = Grid::new(grid.width, grid.height());
        let start_coord = grid.start();
        let start_tile = grid.infer(start_coord);
        let (start_dir, _) = start_tile.get_dirs();
        grid.set(start_coord, start_tile);

        // Clean all the junk out of the grid and detect path winding
        let mut cursor = Cursor::new(&grid, start_coord, start_dir);
        let mut winding = 0;
        loop {
            clean_grid.set(cursor.pos, grid.get(cursor.pos));

            let p1 = cursor.pos;
            let next = cursor.next();
            let p2 = cursor.pos;

            winding += (p2.x - p1.x) * (p2.y + p1.y);

            if next == start_coord {
                break;
            }
        }
        let winding = if winding < 0 {
            Winding::CW
        } else {
            Winding::CCW
        };

        // Follow path and flood fill all adjacent interiors
        let mut cursor = Cursor::new(&grid, start_coord, start_dir);
        let mut interior = HashSet::<IVec2>::new();
        loop {
            let in_dir = cursor.dir;
            let next = cursor.next();
            let out_dir = cursor.dir;

            for dir in [in_dir, out_dir] {
                let flood_start = cursor.pos + dir.inside_dir(winding).delta();
                flood_fill(&clean_grid, flood_start, &mut interior);
            }

            if next == start_coord {
                break;
            }
        }
        interior.len().to_string()
    }
}
