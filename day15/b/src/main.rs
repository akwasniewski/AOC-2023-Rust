use std::io;
fn hash(step: &str) ->u32{
    let mut cur_val = 0;
    for ch in step.chars(){
        let ascii = ch as u32;
        cur_val+=ascii;
        cur_val*=17;
        cur_val%=256;
    }
    cur_val
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal: i32,
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim().split(",");
    let mut sum = 0;
    let mut map: Vec<Vec<Lens>> = vec![Vec::new();256];
    'main_loop: for i in buffer{
        let pos=i.chars().position(|r| r=='=');
        if pos.is_some(){
            let mut i = i.split("=");
            let label = i.next().unwrap();
            let focal = i.next().unwrap().parse().unwrap();
            let hash: usize= hash(label) as usize;
            for i in &mut map[hash]{
                if i.label==label{
                    i.focal=focal;
                    continue 'main_loop;
                }
            }
            map[hash].push(Lens{label,focal});
        }
        else{
            let label = &i.trim()[..i.len()-1];
            let hash: usize = hash(label) as usize;
            map[hash].retain(|&x| x.label != label);
        }
    }
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            sum+=(i+1)*(j+1)*(map[i][j].focal as usize);
        }
    }
    println!("{sum}");
    Ok(())
}