use std::io;
use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq)]
struct MapData{
    group_started: bool,
    springs: Vec<char>,
    groups: Vec<i64>,
}
fn analyze(mut group_started: bool, mut springs: Vec<char>, mut groups: Vec<i64>, map: &mut HashMap<MapData, i64>) -> i64{
    let data = MapData{group_started, springs: springs.clone(), groups: groups.clone()};
    if map.contains_key(&data){
        return map[&data];
    }
    if !springs.is_empty(){
        if springs[0]=='#'{
            if groups.len()==0 || groups[0]==0{
                if !map.contains_key(&data){
                    map.insert(data, 0);
                }
                return 0;
            }
            group_started=true;
            groups[0]-=1;
            springs.remove(0);
            let res = analyze(group_started, springs, groups, map);
            if !map.contains_key(&data){
                map.insert(data, res);
            }
            return res;
        }
        else if springs[0]=='.'{
            if group_started && groups[0] != 0{
                if !map.contains_key(&data){
                    map.insert(data, 0);
                }
                return 0;
            }
            else if group_started{
                groups.remove(0);
                group_started=false;
            }
            springs.remove(0);
            let res = analyze(group_started, springs, groups, map);
            if !map.contains_key(&data){
                map.insert(data, res);
            }
            return res;
        }
        else{
            let mut sum: i64 = 0;
            springs[0]='.';
            sum+=analyze(group_started, springs.clone(), groups.clone(), map);
            springs[0]='#';
            sum+=analyze(group_started, springs, groups, map);
            if !map.contains_key(&data){
                map.insert(data, sum);
            }
            return sum;
        }
    }
    if groups.len()==0{
        if !map.contains_key(&data){
            map.insert(data, 1);
        }
        return 1;
    }
    if !map.contains_key(&data){
        map.insert(data, 0);
    }
    return 0;
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum: i64=0;
    let mut map: HashMap<MapData, i64> = HashMap::new();
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut buffer=buffer.trim().split_whitespace();
        let original_springs: Vec<char>=buffer.next().unwrap().chars().collect();
        let original_groups: Vec<i64>=buffer.next().unwrap().trim().split(',').map(|x| x.parse().unwrap()).collect();
        let mut springs: Vec<char> = Vec::new();
        let mut groups: Vec<i64> = Vec::new();
        for i in 0..5{
            springs.append(&mut original_springs.clone());
            if i==4{
                springs.push('.');
            }
            else{
                springs.push('?');
            }
            groups.append(&mut original_groups.clone());
        }
        let res: i64 = analyze(false,springs, groups, &mut map);
        sum+=res;
    }
    println!("{sum}");
    Ok(())
}