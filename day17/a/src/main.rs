use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos(i32,i32);
fn get_cost(map: & Vec<Vec<i32>>, pos: Pos, rows: i32, columns: i32) -> i32{
    if pos.0>=columns || pos.1>=rows || pos.0<0 || pos.1<0{
        return 1000000;
    }
    return map[pos.1 as usize][pos.0 as usize];
}
fn get_straight(new_dir: i32, old_dir: i32, cur_straight: i32) -> i32{
    if new_dir==old_dir{
        return cur_straight+1;
    }
    return 1;
}
fn dijkstra(map: Vec<Vec<i32>>, mut dist: Vec<Vec<[[i32;3];4]>>) -> i32{
    let rows = map.len() as i32;
    let columns = map[0].len() as i32;
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct QueueItem{
        pos: Pos,
        cost: i32,
        dir: i32,
        straight: i32,
    }
    impl PartialOrd for QueueItem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for QueueItem{
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    let mut que:BinaryHeap<QueueItem> = BinaryHeap::new();
    que.push(QueueItem{pos: Pos(0,0), cost:0, dir: 4, straight: 1});
    while !que.is_empty(){
        let cur = que.pop().unwrap();
        if cur.pos.0>=columns || cur.pos.1>=rows || cur.straight>=4 || cur.pos.0<0 || cur.pos.1<0 || (cur.dir !=4 && cur.cost > dist[cur.pos.1 as usize][cur.pos.0 as usize][cur.dir as usize][cur.straight as usize -1]){
            continue;
        }
        if cur.pos == Pos(columns-1, rows-1){
            return cur.cost;
        }
        if cur.dir!=4{
            dist[cur.pos.1 as usize][cur.pos.0 as usize][cur.dir as usize][cur.straight as usize -1]=cur.cost;
        }
        if cur.dir!=2{
            que.push(QueueItem{pos: Pos(cur.pos.0, cur.pos.1-1), cost: cur.cost+get_cost(&map, Pos(cur.pos.0, cur.pos.1-1), rows, columns), dir: 0, straight: get_straight(0, cur.dir, cur.straight)});
        }
        if cur.dir!=0{
            que.push(QueueItem{pos: Pos(cur.pos.0, cur.pos.1+1), cost: cur.cost+get_cost(&map, Pos(cur.pos.0, cur.pos.1+1), rows, columns), dir: 2, straight: get_straight(2, cur.dir, cur.straight) });
        }
        if cur.dir!=1{
            que.push(QueueItem{pos: Pos(cur.pos.0-1, cur.pos.1), cost: cur.cost+get_cost(&map, Pos(cur.pos.0-1, cur.pos.1), rows, columns), dir: 3, straight: get_straight(3, cur.dir, cur.straight) });
        }
        if cur.dir!=3{
            que.push(QueueItem{pos: Pos(cur.pos.0+1, cur.pos.1), cost: cur.cost+get_cost(&map, Pos(cur.pos.0+1, cur.pos.1), rows, columns), dir: 1, straight: get_straight(1, cur.dir, cur.straight) });
        }
    }
    return 0;
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut dist: Vec<Vec<[[i32;3];4]>> = Vec::new();
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer: Vec<i32>= buffer.trim().chars().map(|x| x.to_string().parse().unwrap()).collect();
        dist.push(vec!([[i32::MAX, i32::MAX, i32::MAX];4]; buffer.len()));
        map.push(buffer);
    }
    let res = dijkstra(map, dist);
    println!("{res}");
    Ok(())
}