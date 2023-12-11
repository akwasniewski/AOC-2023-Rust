use std::io;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pos(i64,i64);
const EXPANSION_SIZE: i64 = 2-1;
fn dist(a: Pos, b: Pos) -> i64{
    (a.0 - b.0).abs()+(a.1-b.1).abs()
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut extra_lines: i64 = 0;
    let mut nodes: Vec<Pos> = Vec::new();
    let mut columns_empty: Vec<i64> = Vec::new();
    for y in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buf_chars: Vec<char> = buffer.trim().chars().collect();
        if columns_empty.len()<buf_chars.len(){
            columns_empty.resize(buf_chars.len(),0);
        }
        let mut found_hash: bool = false;
        for i in 0..buf_chars.len(){
            if buf_chars[i]=='#'{
                found_hash=true;
                nodes.push(Pos(i as i64, extra_lines+ y as i64));
                columns_empty[i]=1;
            }
        }
        if !found_hash{
            extra_lines+=EXPANSION_SIZE;
        }
    }
    let mut cur_added: i64=0;
    for i in 0..columns_empty.len(){
        if columns_empty[i]!=1{
            columns_empty[i]=cur_added;
            cur_added+=EXPANSION_SIZE;
        }
        else{
            columns_empty[i]=cur_added;
        }
    }
    for i in 0..nodes.len(){
        nodes[i].0=nodes[i].0+columns_empty[nodes[i].0 as usize];
    }
    let mut sum: i64=0;
    for i in 0..nodes.len(){
        for j in i+1..nodes.len(){
            let dist=dist(nodes[i], nodes[j]);
            sum+=dist;
        }
    }
    println!("{sum}");
    Ok(())
}