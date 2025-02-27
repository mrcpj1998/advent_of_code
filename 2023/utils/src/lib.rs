use regex::Regex;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;

//static PROD: bool = false;
//static DAY: &str = "day1";
pub fn load_input(prod: bool, day: &str) -> String {
    let path: String = match prod {
        true => format!("inputs/{}.input", day),
        false => format!("inputs/{}.example", day),
    };
    let mut file = match File::open(Path::new(&path)) {
        Ok(s) => s,
        Err(why) => panic!("Error in reading {path}\n{why}\n"),
    };
    let mut out = String::new();
    match file.read_to_string(&mut out) {
        Ok(_) => (),
        Err(why) => panic!("BBBBBBBBB {}\n", why),
    }
    return out;
}

pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub maze: Vec<Vec<u8>>,
}

pub fn load_parts(inp: &str) -> Vec<&str> {
    inp.split("\n\n").collect()
}

pub fn parse_numbers(inp: &str) -> Vec<isize> {
    let numregex = Regex::new(r"(-?\d+)").unwrap();
    numregex
        .captures_iter(inp)
        .map(|c| {
            let (_, [num]) = c.extract();
            num
        })
        .map(|c| {
            c.parse::<isize>()
                .expect(&format!("Couldn't parse '{}' as isize", c))
        })
        .collect()
}

/// Returns: Height, width, vecvec
pub fn load_2darr(inp: &str) -> Maze {
    let height: usize = inp.lines().count();
    let width: usize = inp.lines().next().unwrap().len();
    let mut arr: Vec<Vec<u8>> = Vec::new();
    for l in inp.lines() {
        arr.push(Vec::from(l));
    }
    return Maze {
        height,
        width,
        maze: arr,
    };
}
pub const COMPASS_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub const GRID_DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];
pub fn neighbors_2d(r: usize, c: usize, maze: &Maze) -> Vec<(u8, i32, i32)> {
    let w = maze.width as i32;
    let h = maze.height as i32;
    let mut neigbors: Vec<(u8, i32, i32)> = Vec::new();
    for (dy, dx) in GRID_DIRECTIONS.iter() {
        let (nr, nc) = ((r as i32) + dy, (c as i32) + dx);
        if 0 <= nr && nr < h && 0 <= nc && nc < w {
            neigbors.push((maze.maze[nr as usize][nc as usize], nr, nc));
        }
    }
    return neigbors;
}

pub fn strvec<A>(arr: &Vec<A>, sep: &str) -> String
where
    A: Display,
{
    let mut s = String::new();
    for i in 0..arr.len() {
        s.push_str(arr[i].to_string().as_str());
        if i != arr.len() - 1 {
            s.push_str(sep)
        }
    }
    return s;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
