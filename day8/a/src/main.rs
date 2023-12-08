use std::io;
use std::collections::HashMap;
struct MapInfo{
    left: String,
    right: String
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    buffer=String::new();
    io::stdin().read_line(&mut buffer)?;
    let instructions: Vec<char> = buffer.trim().chars().collect();
    let mut map: HashMap<String, MapInfo> = HashMap::new();
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut buffer=buffer.split("=");
        let current = buffer.next().unwrap().trim();
        let neighbours = buffer.next().unwrap();
        let neighbours = neighbours.replace(&['(',')'][..], "");
        let mut neighbours = neighbours.split(",");
        let left = neighbours.next().unwrap().trim().to_string();
        let right = neighbours.next().unwrap().trim().to_string();
        map.insert(current.to_string(), MapInfo{left,right});
    }
    let mut pos: &String = &"AAA".to_string();
    let mut steps=0;
    'main_loop: loop{
        for i in &instructions{
            if pos=="ZZZ"{
                break 'main_loop;
            }
            steps+=1;

            if i==&'L'{
                pos=&map[pos].left;
            }
            else{
                pos=&map[pos].right;
            }
        }
    }
    println!("{steps}");
    Ok(())
}