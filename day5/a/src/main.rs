use std::io;
#[derive(Debug, Clone, Copy)]
struct MapInfo{
    source_begin: usize,
    destination_begin: usize,
    length: usize,
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut seeds: Vec<usize> = Vec::new();
    let mut maps: Vec<Vec<MapInfo>> = vec![Vec::new(); 7];
    let mut input_state=0;
    for _ in 0..line_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.len()==1{
            input_state+=1;
            continue;
        }
        if input_state!=0 && !buffer.chars().next().unwrap().to_string().parse::<i32>().is_ok(){
            continue;
        }
        if input_state == 0{
            let new_seeds=buffer.split(":").last().unwrap().split(" ");
            for seed in new_seeds{
                if seed==""{
                    continue;
                }
                let seed=seed.trim().parse().unwrap();
                seeds.push(seed);
            }
            continue;
        }
        let vals = buffer.split(" ").collect::<Vec<&str>>();
        let new_map: MapInfo = MapInfo{
            destination_begin: vals[0].trim().parse().unwrap(),
            source_begin: vals[1].trim().parse().unwrap(),
            length: vals[2].trim().parse().unwrap(),
        };
        maps[input_state-1].push(new_map);
    }
    let mut min = usize::MAX;
    for seed in seeds{
        let mut cur_val=seed;
        'type_loop: for map_type in &maps{
            for map in map_type{
                if cur_val>=map.source_begin && cur_val<map.source_begin+map.length{
                    cur_val=map.destination_begin+(cur_val-map.source_begin);
                    continue 'type_loop;
                }
            }
        }
        if cur_val<min{
            min=cur_val;
        }
    }
    println!("{min}");
    Ok(())
}