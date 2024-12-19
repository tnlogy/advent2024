use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let input = include_str!("../../indata/day18.txt");
    let coords = input.lines().map(|l| {
        let (a, b) = l.split_once(",").unwrap();
        (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
    }).collect::<Vec<_>>();


    let width = *coords.iter().map(|(x, _)| x).max().unwrap();
    let height = *coords.iter().map(|(_, y)| y).max().unwrap();

    println!("width {width}, height {height}");
    println!("coords = {:?}", coords.len()-3);

    for i in 1024..coords.len() {
        let mut grid = vec![vec!['.'; width + 1]; height + 1];
        for (x, y) in &coords[0..i] {
            grid[*y][*x] = '#';
        }
        println!("search {i} {:?}", coords[i-1]);
        let found = search(&mut grid, width, height);
        if !found {
            println!("no exit at {i} {:?}", coords[i-1]);
            break;
        }

    }
//    print_grid(&grid);

}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("{}", grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n"));
}


fn search(grid: &mut Vec<Vec<char>>, width: usize, height: usize) -> bool {
    let start = (0,0);
    let end = (width, height);
    println!("Find path from {:?} -> {:?}", start, end);

    let mut visited: HashMap<State, SearchItem> = HashMap::new();
    let mut pq = BinaryHeap::new();
    let state = State { x: start.0, y: start.1 };
    let mut best_cost = usize::MAX;
    let mut spectators: HashSet<(usize, usize)> = HashSet::new();
    pq.push(SearchItem { state, cost: 0, rank: 0 }); // , visited: vec![]});

    while let Some(p) = pq.pop() {
        if p.state.x == end.0 && p.state.y == end.1 {
            println!("part1 cost:{:?}", p.cost);
            if p.cost > best_cost {

                println!("All best solutions found");
                break;
            }
            best_cost = p.cost;
            return true;
        }

        if !visited.contains_key(&p.state) {
            visited.insert(p.state.clone(), p.clone());
            for n in actions(&p, &grid) {
                let mh_dist = end.0.abs_diff(n.state.x) + end.1.abs_diff(n.state.y);
                pq.push(SearchItem { rank: n.cost + mh_dist, ..n });
            }
        }
    }
    false
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
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SearchItem {
    cost: usize,
    rank: usize,

    state: State,
//    visited: Vec<State>
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
//    let mut visited = pos.visited.clone();
//    visited.push(pos.state.clone());

    for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
        let (x, y) = (pos.state.x as i32 + dx, pos.state.y as i32 + dy);
        if x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32 {
            let (x, y) = (x as usize, y as usize);
            if grid[y][x] != '#' {
                let state = State { x, y };
                res.push(SearchItem { state, cost: pos.cost + 1, rank: 0 }); // , visited: visited.clone() });
            }
        }
    }
    res
}
