#[derive(Debug, Clone)]
struct Node {
    pos: usize,
    size: usize,
}

fn main() {
    let input = include_str!("../../indata/day9.txt");
    let contents = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();

    for part in [1, 2] {
        let mut data: Vec<Node> = Vec::new();
        let mut data_id_list: Vec<i32> = vec![]; // index of Node
        let mut index = 0;
        for i in (0..contents.len()).step_by(2) {
            //println!("{} {}", i, data.len());
            let size = contents[i];
            let free_space = if i + 1 == contents.len() { 0 } else { contents[i + 1] };
            let n = Node {pos: index, size};
            data.push(n.clone());
            for _ in 0..n.size {
                data_id_list.push((data.len() - 1) as i32);
            }
            for _ in 0..free_space {
                data_id_list.push(-1);
            }
            index += size + free_space;
        }

        let mut copy_index = data.len() - 1;
        let index = data[0].pos;
        let mut free_space = true;
        while free_space {
            let size = data[copy_index].size;
            let ci = data[copy_index].pos;
            if part == 2 {
                let free_index = data_id_list[index..].windows(size).position(|window| window.iter().all(|&item| item == -1));
                if let Some(i) = free_index {
                    if i <= ci {
                        // println!("free index {} {}", i, copy_index);
                        for s in 0..size {
                            data_id_list[i + s] = copy_index as i32;
                            data_id_list[ci + s] = -1;
                        }
                    }
                }
            } else {
                for s in 0..size {
                    let free_index = data_id_list[index..].iter().position(|&x| x == -1);
                    if let Some(i) = free_index {
                        if i <= ci {
                            data_id_list[i] = copy_index as i32;
                            data_id_list[ci + s] = -1;
                        } else {
                            free_space = false;
                            break;
                        }
                    } else {
                        free_space = false;
                        break;
                    }
                }
            }
            if copy_index == 1 {
                break;
            }
            copy_index -= 1;
        }
        let res = data_id_list.iter().enumerate().map(|(i, &x)| if x != -1 { i*x as usize } else { 0 }).sum::<usize>();
        assert_eq!(res, if part == 1 { 6154342787400 } else { 6183632723350 });
        println!("part{} {:?}", part, res);
    }
}