use std::io;
use std::cmp::min;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pos(usize,usize);
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Dir(i8,i8);
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
    let mut cur_out=Dir(0,0);
    if graph[s_pos.1][s_pos.0-1]=='-' || graph[s_pos.1][s_pos.0-1]=='L' || graph[s_pos.1][s_pos.0-1]=='F'{
        cur_pos = Pos(s_pos.0-1, s_pos.1);
        cur_out=Dir(0,1);
    }
    else if graph[s_pos.1][s_pos.0+1]=='-' || graph[s_pos.1][s_pos.0+1]=='J' || graph[s_pos.1][s_pos.0+1]=='7'{
        cur_pos = Pos(s_pos.0+1, s_pos.1);
        cur_out=Dir(0,1);
    }
    else if graph[s_pos.1-1][s_pos.0]=='|' || graph[s_pos.1-1][s_pos.0]=='F' || graph[s_pos.1-1][s_pos.0]=='7'{
        cur_pos = Pos(s_pos.0, s_pos.1-1);
        cur_out=Dir(1,0);
        graph[s_pos.1][s_pos.0]='>';
    }
    else if graph[s_pos.1+1][s_pos.0]=='|' || graph[s_pos.1+1][s_pos.0]=='L' || graph[s_pos.1+1][s_pos.0]=='J'{
        cur_pos = Pos(s_pos.0, s_pos.1+1);
        cur_out=Dir(1,0);
        graph[s_pos.1][s_pos.0]='>';
    }
    while cur_pos != s_pos{
        let mut new_pos;
        let mut new_out;
        match graph[cur_pos.1][cur_pos.0]{
            '|' => {
                new_pos=Pos(cur_pos.0, cur_pos.1-1);
                new_out=cur_out;
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0, cur_pos.1+1);
                }
            },
            '-' => {
                new_pos=Pos(cur_pos.0-1, cur_pos.1);
                new_out=cur_out;
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0+1, cur_pos.1);
                }
            },
            'L' => {
                new_pos=Pos(cur_pos.0, cur_pos.1-1);
                new_out=Dir(-cur_out.1,-cur_out.0);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0+1, cur_pos.1);
                }
            },
            'J' => {
                new_pos=Pos(cur_pos.0, cur_pos.1-1);
                new_out=Dir(cur_out.1,cur_out.0);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0-1, cur_pos.1);
                }
            },
            '7' => {
                new_pos=Pos(cur_pos.0-1, cur_pos.1);
                new_out=Dir(-cur_out.1,-cur_out.0);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0, cur_pos.1+1);
                }
            },
            'F' => {
                new_pos=Pos(cur_pos.0+1, cur_pos.1);
                new_out=Dir(cur_out.1,cur_out.0);
                if new_pos==prev_pos{
                    new_pos=Pos(cur_pos.0, cur_pos.1+1);
                }
            },
            _ => panic!("error while traversing the graph"),
        }
        if new_out.0==-1{
            graph[cur_pos.1][cur_pos.0]='<';
        }
        else if new_out.0==1{
            graph[cur_pos.1][cur_pos.0]='>';
        }
        else if graph[cur_pos.1][cur_pos.0]!='-'{
            if cur_out.0==-1{
                graph[cur_pos.1][cur_pos.0]='<';
            }
            else if cur_out.0==1{
                graph[cur_pos.1][cur_pos.0]='>';
            }
        }
        else{
            graph[cur_pos.1][cur_pos.0]='X';
        }
        prev_pos=cur_pos;
        cur_pos=new_pos;
        cur_out=new_out;
    }
    if graph[s_pos.1][s_pos.0] =='S' && graph[prev_pos.1][prev_pos.0]!='X'{
        if cur_out.0==-1{
            graph[cur_pos.1][cur_pos.0]='<';
        }
        else if cur_out.0==1{
            graph[cur_pos.1][cur_pos.0]='>';
        }
    }
    let mut count_in=0;
    let mut which_first='?';
    let mut i = 0;
    let mut is_in=false;
    for row in graph{
        for cur in row{
            if (cur=='<' || cur=='>') && which_first=='?'{
                which_first=cur;
            }
            if cur==which_first{
                is_in=true;
            }
            else if cur=='<' || cur=='>'{
                is_in=false;
            }
            else if cur!='X' && is_in==true{
                count_in+=1;   
            }
        }
    }
    println!("{count_in}");
    Ok(())
}