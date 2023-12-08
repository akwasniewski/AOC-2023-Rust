use std::io;
use std::collections::HashMap;
struct MapInfo{
    left: String,
    right: String
}
fn gcd(a: u64, b: u64) -> u64{
    if b==0{
        a
    }
    else{
        gcd(b, a%b)
    }
}
fn lcm(a: u64, b: u64)->u64{
    a*b/gcd(a,b)
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    buffer=String::new();
    io::stdin().read_line(&mut buffer)?;
    let instructions: Vec<char> = buffer.trim().chars().collect();
    let mut map: HashMap<String, MapInfo> = HashMap::new();
    let mut starting_points: Vec<String>=Vec::new();
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut buffer=buffer.split("=");
        let current = buffer.next().unwrap().trim().to_string();
        if current.chars().last().unwrap()=='A'{
            starting_points.push(current.clone());
        }
        let neighbours = buffer.next().unwrap();
        let neighbours = neighbours.replace(&['(',')'][..], "");
        let mut neighbours = neighbours.split(",");
        let left = neighbours.next().unwrap().trim().to_string();
        let right = neighbours.next().unwrap().trim().to_string();
        map.insert(current, MapInfo{left,right});
    }
    let mut res: u64=1;
    for point in 0..starting_points.len(){
        let mut steps: u64=0;
        'main_loop: loop{
            for i in &instructions{
                if starting_points[point].chars().last().unwrap()=='Z'{
                    break 'main_loop;
                }
                steps+=1;
                if i==&'L'{
                    starting_points[point]=map[&starting_points[point]].left.clone();
                }
                else{
                    starting_points[point]=map[&starting_points[point]].right.clone();
                }
            }
        }
        res=lcm(res,steps);
    }
    println!("{res}");
    Ok(())
}