use std::io;
struct Node{
    label: char,
    visited: [bool; 4],
    energized: bool,
}
struct Pos(i64,i64);
fn dfs(map: &mut Vec<Vec<Node>>, pos: Pos, mut dir: usize){
    if pos.0<0 || pos.1<0 || (pos.1 as usize)>=map.len() || (pos.0 as usize)>=map[0].len() || map[pos.1 as usize][pos.0 as usize].visited[dir]==true{
        return;
    }
    map[pos.1 as usize][pos.0 as usize].energized=true;
    map[pos.1 as usize][pos.0 as usize].visited[dir]=true;
    match map[pos.1 as usize][pos.0 as usize].label{
        '.' => {
            let next_pos: Pos;
            match dir{
                0 => next_pos = Pos(pos.0, pos.1-1),
                1 => next_pos = Pos(pos.0+1, pos.1),
                2 => next_pos = Pos(pos.0, pos.1+1),
                3 => next_pos = Pos(pos.0-1, pos.1),
                _ => panic!("dfs error"),
            };
            dfs(map, next_pos, dir)},
        '/' => {
            let next_pos: Pos;
            match dir{
                0 => {
                    next_pos = Pos(pos.0+1, pos.1);
                    dir=1;
                }
                1 => {
                    next_pos = Pos(pos.0, pos.1-1);
                    dir=0;
                }
                2 => {
                    next_pos = Pos(pos.0-1, pos.1);
                    dir=3;
                }
                3 => {
                    next_pos = Pos(pos.0, pos.1+1);
                    dir=2;
                }
                _ => panic!("dfs error"),
            };
            dfs(map, next_pos, dir)},
        '\\' => {
            let next_pos: Pos;
            match dir{
                0 => {
                    next_pos = Pos(pos.0-1, pos.1);
                    dir=3;
                }
                1 => {
                    next_pos = Pos(pos.0, pos.1+1);
                    dir=2;
                }
                2 => {
                    next_pos = Pos(pos.0+1, pos.1);
                    dir=1;
                }
                3 => {
                    next_pos = Pos(pos.0, pos.1-1);
                    dir=0;
                }
                _ => panic!("dfs error"),
            };
            dfs(map, next_pos, dir)
        },
        '|' => {
            match dir{
            0 => {
                let next_pos = Pos(pos.0, pos.1-1);
                dfs(map, next_pos, dir);
            }
            1 | 3=> {
                let next_pos = Pos(pos.0, pos.1-1);
                dfs(map, next_pos, 0);
                let next_pos = Pos(pos.0, pos.1+1);
                dfs(map, next_pos, 2);
            }
            2 => {
                let next_pos = Pos(pos.0, pos.1+1);
                dfs(map, next_pos, dir);
            }
            _ => panic!("dfs error"),
            } 
        },
        '-' => {
            match dir{
            0 | 2=> {
                let next_pos = Pos(pos.0-1, pos.1);
                dfs(map, next_pos, 3);
                let next_pos = Pos(pos.0+1, pos.1);
                dfs(map, next_pos, 1);
            }
            1=> {
                let next_pos = Pos(pos.0+1, pos.1);
                dfs(map, next_pos, dir);
            }
            3 => {
                let next_pos = Pos(pos.0-1, pos.1);
                dfs(map, next_pos, dir);
            }
            _ => panic!("dfs error"),
            } 
        },
        _ => panic!("dfs error"),
    }
}
fn energize(map: &mut Vec<Vec<Node>>, pos: Pos, dir: usize) -> u32{
    dfs(map, pos, dir);
    let mut sum = 0;
    for row in map{
        for node in row{
            if node.energized{
                sum+=1;
                node.energized=false;
                node.visited = [false;4];
            }
        }
    }
    sum
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut map: Vec<Vec<Node>> = Vec::new();
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer: Vec<Node> = buffer.trim().chars().map(|x| Node{label: x, visited: [false; 4], energized: false}).collect();
        map.push(buffer);
    }
    let mut best = 0;

    for i in 0..map.len(){
        let res = energize(&mut map, Pos(0,i as i64), 1);
        if res>best{
            best=res;
        }
        let res= energize(&mut map, Pos((line_count-1) as i64, i as i64), 3);
        if res>best{
            best=res;
        }
    }
    let map_width = map[0].len();
    for i in 0..map[0].len(){
        let res = energize(&mut map, Pos(i as i64, 0), 2);
        if res>best{
            best=res;
        }
        let res= energize(&mut map, Pos(i as i64, (map_width-1) as i64, ), 0);
        if res>best{
            best=res;
        }
    }
    println!("{best}");
    Ok(())
}