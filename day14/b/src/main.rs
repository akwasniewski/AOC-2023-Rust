use std::io;
use std::collections::HashMap;
fn tilt_north(map: &mut Vec<Vec<char>>){
    for i in 0..map.len(){
        for j in 0..map[0].len(){
            if map[i][j]=='O'{
                let mut cur_i=i;
                while cur_i != 0 && map[cur_i-1][j]=='.'{
                    cur_i-=1;
                }
                map[i][j]='.';
                map[cur_i][j]='O';

            }
        }
    }
}
fn tilt_south(map: &mut Vec<Vec<char>>){
    for i in (0..map.len()).rev(){
        for j in 0..map[0].len(){
            if map[i][j]=='O'{
                let mut cur_i=i;
                while cur_i != map.len()-1 && map[cur_i+1][j]=='.'{
                    cur_i+=1;
                }
                map[i][j]='.';
                map[cur_i][j]='O';
            }
        }
    }
}
fn tilt_west(map: &mut Vec<Vec<char>>){
    for i in 0..map.len(){
        for j in 0..map[0].len(){
            if map[i][j]=='O'{
                let mut cur_j=j;
                while cur_j!=0 && map[i][cur_j-1]=='.'{
                    cur_j-=1;
                }
                map[i][j]='.';
                map[i][cur_j]='O';
            }
        }
    }
}
fn tilt_east(map: &mut Vec<Vec<char>>){
    for i in 0..map.len(){
        for j in (0..map[0].len()).rev(){
            if map[i][j]=='O'{
                let mut cur_j=j;
                while cur_j!=map[0].len()-1 && map[i][cur_j+1]=='.'{
                    cur_j+=1;
                }
                map[i][j]='.';
                map[i][cur_j]='O';
            }
        }
    }
}
fn rotate(map: &mut Vec<Vec<char>>){
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
}
fn get_load(map: Vec<Vec<char>>, row_count: usize) -> usize{
   let mut sum=0;
    for row in 0..map.len(){
        let buffer=&map[row];
        for i in 0..buffer.len(){
            if buffer[i]=='O'{
                sum+=row_count-row;
            }
        }
    }
    return sum;
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut buffer = buffer.trim().split_whitespace();
    let row_count: usize = buffer.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..row_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer: Vec<char> = buffer.trim().chars().collect();
        map.push(buffer);
    }
    let mut rotations = 0;
    let mut hash_map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    while rotations==0 || !hash_map.contains_key(&map){
        hash_map.insert(map.clone(), rotations);
        rotate(&mut map);
        rotations+=1;
    }
    let cycle_length=rotations-hash_map[&map];
    let modulo = (1000000000-hash_map[&map]) % cycle_length;
    println!("{rotations} {} {modulo}",hash_map[&map]);

    for _ in 0..modulo{
        rotate(&mut map);
    }
    let res = get_load(map, row_count);
    println!("{res}");
    Ok(())
}