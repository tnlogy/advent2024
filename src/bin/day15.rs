fn main() {
    let input = include_str!("../../indata/day15.txt");
    let (mut map, moves) = input.split_once("\n\n").unwrap();
    let enlarged_map = &map
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.").to_string();

    for part in [1, 2] {
        if part == 2 {
            map = enlarged_map;
        }

        let mut grid = map
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();
        println!("{}", grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"));

        for m in moves.chars() {
            if m == '\n' { continue; }
            let bot_pos = find_bot(&grid);
            let dir = match m {
                '>' => (1, 0),
                '<' => (-1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => panic!("incorrect move"),
            };
            if part == 1 {
                push(&mut grid, bot_pos, dir, false);
            } else {
                let moved = push(&mut grid, bot_pos, dir, true);
                if moved {
                    push(&mut grid, bot_pos, dir, false);
                }
            }
        }
        println!("{}", grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"));
        println!("part{part} {}", sum_of_boxes(&grid))
    }
}

fn sum_of_boxes(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' || grid[y][x] == '[' {
                sum += 100 * y + x;
            }
        }
    }
    sum
}

fn push(grid: &mut Vec<Vec<char>>, pos: (usize, usize), dir: (i32, i32), dry_run: bool) -> bool {
    // dry_run is used in part2 to first check if the move is possible, and then perform it.
    let e = grid[pos.1][pos.0];
    if e == '.' { return true; }
    if e == '#' { return false; }

    if dir.1 != 0 && e == '[' {
        let pushed_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        let pushed_pos2 = (pos.0 as i32 + dir.0 + 1, pos.1 as i32 + dir.1);
        let p0 = push(grid, (pushed_pos.0 as usize, pushed_pos.1 as usize), dir, dry_run);
        let p1 = push(grid, (pushed_pos2.0 as usize, pushed_pos2.1 as usize), dir, dry_run);
        if p0 && p1 {
            if !dry_run {
                grid[pos.1][pos.0] = '.';
                grid[pos.1][pos.0 + 1] = '.';
                grid[pushed_pos.1 as usize][pushed_pos.0 as usize] = '[';
                grid[pushed_pos2.1 as usize][pushed_pos2.0 as usize] = ']';
            }
            true
        } else {
            false
        }
    } else if dir.1 != 0 && e == ']' {
        let pushed_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        let pushed_pos2 = (pos.0 as i32 + dir.0 - 1, pos.1 as i32 + dir.1);
        let p0 = push(grid, (pushed_pos.0 as usize, pushed_pos.1 as usize), dir, dry_run);
        let p1 = push(grid, (pushed_pos2.0 as usize, pushed_pos2.1 as usize), dir, dry_run);
        if p0 && p1 {
            if !dry_run {
                grid[pos.1][pos.0] = '.';
                grid[pos.1][pos.0 - 1] = '.';
                grid[pushed_pos.1 as usize][pushed_pos.0 as usize] = ']';
                grid[pushed_pos2.1 as usize][pushed_pos2.0 as usize] = '[';
            }
            true
        } else {
            false
        }
    } else {
        let pushed_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        if push(grid, (pushed_pos.0 as usize, pushed_pos.1 as usize), dir, dry_run) {
            if !dry_run {
                grid[pos.1][pos.0] = '.';
                grid[pushed_pos.1 as usize][pushed_pos.0 as usize] = e;
            }
            true
        } else {
            false
        }
    }
}

fn find_bot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                return (x, y);
            }
        }
    }
    panic!("bot not found");
}
