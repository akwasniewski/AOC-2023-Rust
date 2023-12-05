// Due to time scheduling problems (I have a math exam toomorow) here is a bruteforce approach
// On my Laptop with --release flag it runs in about 3 minutes
use std::io;
#[derive(Debug, Clone, Copy)]
struct MapInfo{
    source_begin: usize,
    destination_begin: usize,
    length: usize,
}
struct SeedInfo{
    start: usize,
    length: usize,
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut seeds: Vec<SeedInfo> = Vec::new();
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
            let new_seeds=buffer.split(":").last().unwrap().split(" ").collect::<Vec<&str>>();
            for i in (1..new_seeds.len()).step_by(2){
                let start=new_seeds[i].trim().parse().unwrap();
                let length=new_seeds[i+1].trim().parse().unwrap();
                seeds.push(SeedInfo{start, length});
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
        for seed_num in seed.start..seed.start+seed.length{
            let mut cur_val=seed_num;
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
    }
    println!("{min}");
    Ok(())
}