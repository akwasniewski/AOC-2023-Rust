use std::io;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pos(usize,usize);
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut graph: Vec<Vec<char>>=Vec::new();
    let mut s_pos: Pos=Pos(0,0);
    for y in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer: Vec<char> = buffer.trim().chars().collect();
        let s_x=buffer.iter().position(|&r| r=='S');
        if s_x.is_some(){
            s_pos=Pos(s_x.unwrap(), y);
        }
        graph.push(buffer);
    }
    let mut path: Vec<Pos> = Vec::new();
    let mut cur_pos=Pos(0,0);
    let mut prev_pos=s_pos;
    if graph[s_pos.1][s_pos.0-1]=='-' || graph[s_pos.1][s_pos.0-1]=='L' || graph[s_pos.1][s_pos.0-1]=='F'{
        cur_pos = Pos(s_pos.0-1, s_pos.1);
    }
    else if graph[s_pos.1][s_pos.0+1]=='-' || graph[s_pos.1][s_pos.0+1]=='J' || graph[s_pos.1][s_pos.0+1]=='7'{
        cur_pos = Pos(s_pos.0+1, s_pos.1);
    }
    else if graph[s_pos.1-1][s_pos.0]=='|' || graph[s_pos.1-1][s_pos.0]=='F' || graph[s_pos.1-1][s_pos.0]=='7'{
        cur_pos = Pos(s_pos.0, s_pos.1-1);
    }
    else if graph[s_pos.1+1][s_pos.0]=='|' || graph[s_pos.1+1][s_pos.0]=='L' || graph[s_pos.1+1][s_pos.0]=='J'{
        cur_pos = Pos(s_pos.0, s_pos.1+1);
    }
    
    while cur_pos!=s_pos{
        let mut new_pos;
        match graph[cur_pos.1][cur_pos.0]{
            '|' => {
                new_pos=Pos(cur_pos.0, cur_pos.1-1);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0, cur_pos.1+1);
                }
            },
            '-' => {
                new_pos=Pos(cur_pos.0-1, cur_pos.1);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0+1, cur_pos.1);
                }
            },
            'L' => {
                new_pos=Pos(cur_pos.0, cur_pos.1-1);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0+1, cur_pos.1);
                }
            },
            'J' => {
                new_pos=Pos(cur_pos.0, cur_pos.1-1);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0-1, cur_pos.1);
                }
            },
            '7' => {
                new_pos=Pos(cur_pos.0-1, cur_pos.1);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0, cur_pos.1+1);
                }
            },
            'F' => {
                new_pos=Pos(cur_pos.0+1, cur_pos.1);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0, cur_pos.1+1);
                }
            },
            _ => panic!("error while traversing the graph"),
        }
        path.push(cur_pos);
        prev_pos=cur_pos;
        cur_pos=new_pos;
    }
    let res=(path.len() as f32/2.0).ceil();
    println!("{res}");
    Ok(())
}