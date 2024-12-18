use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let input = include_str!("../../indata/day16.txt");
    let mut grid = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    let start = find_char(&grid, 'S');
    let end = find_char(&grid, 'E');
    println!("Find path from {:?} -> {:?}", start, end);

    let mut visited: HashMap<State, SearchItem> = HashMap::new();
    let mut pq = BinaryHeap::new();
    let state = State { x: start.0, y: start.1, dx: 1, dy: 0 };
    let mut best_cost = usize::MAX;
    let mut spectators: HashSet<(usize, usize)> = HashSet::new();
    pq.push(SearchItem { state, cost: 0, rank: 0, visited: vec![]});

    while let Some(p) = pq.pop() {
        if p.state.x == end.0 && p.state.y == end.1 {
            if p.cost > best_cost {
                println!("part1 cost:{:?}", p.cost);
                println!("All best solutions found");
                break;
            }
            best_cost = p.cost;
            spectators.extend(p.visited.iter().map(|v| (v.x, v.y)).collect::<Vec<(usize, usize)>>());
        }

        if !visited.contains_key(&p.state) || visited.get(&p.state).unwrap().rank >= p.rank {
            visited.insert(p.state.clone(), p.clone());
            for n in actions(&p, &grid) {
                let mh_dist = end.0.abs_diff(n.state.x) + end.1.abs_diff(n.state.y);
                pq.push(SearchItem { rank: n.cost + mh_dist, ..n });
            }
        }
    }
    // print_grid(&grid);
    let places = spectators.len() + 1;
    println!("part2: {}", places);
}

fn find_char(grid: &Vec<Vec<char>>, e: char) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == e {
                return (x, y);
            }
        }
    }
    panic!("bot not found");
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct State {
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
}
impl State {
    fn turn_right(&self) -> Self {
        let (dx, dy) = match (self.dx, self.dy) {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => panic!("incorrect direction"),
        };
        Self { dx, dy, ..*self }
    }
    fn turn_left(&self) -> Self {
        let (dx, dy) = match (self.dx, self.dy) {
            (0, 1) => (1, 0),
            (-1, 0) => (0, 1),
            (0, -1) => (-1, 0),
            (1, 0) => (0, -1),
            _ => panic!("incorrect direction"),
        };
        Self { dx, dy, ..*self }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SearchItem {
    cost: usize,
    rank: usize,

    state: State,
    visited: Vec<State>
}
impl Ord for SearchItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank).reverse() // Higher rank gets higher priority
    }
}
impl PartialOrd for SearchItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn actions(pos: &SearchItem, grid: &Vec<Vec<char>>) -> Vec<SearchItem> {
    let mut res: Vec<SearchItem> = Vec::new();
    let mut visited = pos.visited.clone();
    visited.push(pos.state.clone());

    let (x, y) = ((pos.state.x as i32 + pos.state.dx) as usize, (pos.state.y as i32 + pos.state.dy) as usize);
    if grid[y as usize][x as usize] != '#' {
        let state = State { x, y, ..pos.state };
        res.push(SearchItem { state, cost: pos.cost + 1, rank: 0, visited: visited.clone() });
    }
    res.push(SearchItem { state: pos.state.turn_left(), cost: pos.cost + 1000, rank: 0,  visited: visited.clone() });
    res.push(SearchItem { state: pos.state.turn_right(), cost: pos.cost + 1000, rank: 0,  visited: visited.clone() });
    res
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("{}", grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n"));
}