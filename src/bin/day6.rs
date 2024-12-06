use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct Pos { x: i32, y: i32, }
impl Pos {
    pub(crate) fn update(&self, d: &Dir) -> Self {
        Self { x: self.x + d.x, y: self.y + d.y }
    }
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Pos {}
impl Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, Clone)]
struct Dir { x: i32, y: i32, }
impl Dir {
    fn turn (&self) -> Dir {
        if self.x == 0 && self.y == -1 {
            Dir { x: 1, y: 0 }
        } else if self.x == 1 && self.y == 0 {
            Dir { x: 0, y: 1 }
        } else if self.x == 0 && self.y == 1 {
            Dir { x: -1, y: 0 }
        } else {
            Dir { x: 0, y: -1 }
        }
    }
}
impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone)]
struct State { pos: Pos, dir: Dir }

fn find_start_pos(grid: Vec<Vec<char>>) -> Pos {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return Pos { x: x as i32, y: y as i32 };
            }
        }
    }
    panic!("No start pos found!");
}

fn main() {
    let input = include_str!("../../indata/day6.txt");
    let mut grid = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut start_pos = find_start_pos(grid.clone());
    let mut guard_steps = 1;
    let mut guard_positions: Vec<Pos> = vec![];
    let mut n_loops = 0;
    let mut nth_run = 0;
    loop {
        let mut grid = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
        let mut dir = Dir { x: 0, y: -1, };
        let mut pos = start_pos.clone();
        let mut path = vec![State { pos: pos.clone(), dir: dir.clone() }];

        if nth_run > 0 { // now searching for loops
            if nth_run >= guard_positions.len() {
                break; // all done
            }
            let s = &guard_positions[nth_run];
            grid[s.y as usize][s.x as usize] = 'O';
        }
        loop {
            let new_pos = pos.update(&dir);
            if new_pos.x < 0 || new_pos.x as usize >= grid[0].len() || new_pos.y < 0 || new_pos.y as usize >= grid.len() {
                if nth_run == 0 { // first walk is complete, store unique positions of the guard
                    println!("part1 {}", guard_steps);
                    guard_positions = path.iter().map(|s| s.pos.clone()).collect::<HashSet<Pos>>().into_iter().collect::<Vec<Pos>>();
                }
                nth_run += 1;
                break
            }
            let walk_into = grid[new_pos.y as usize][new_pos.x as usize];
            match walk_into {
                '#'|'O' => dir = dir.turn(),
                _ => {
                    if walk_into != 'X' {
                        guard_steps += 1;
                    }
                    grid[pos.y as usize][pos.x as usize] = 'X';
                    pos = pos.update(&dir);
                    if path.iter().find(|s| s.pos == pos && s.dir == dir ).is_some() {
                        // found a loop since pos and dir is already visited
                        // println!("{}", grid.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
                        n_loops += 1;
                        nth_run += 1;
                        break;
                    }
                    path.push(State { pos: pos.clone(), dir: dir.clone() });
                }
            }
        }
    }
    println!("part2 {}", n_loops);
}