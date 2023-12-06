use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let times: Vec<f32>=buffer.trim().split(":").last().unwrap().trim().split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let distances: Vec<f32>=buffer.trim().split(":").last().unwrap().trim().split_whitespace().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    let mut res: i32=1;
    for i in 0..times.len(){
        let delta=(times[i]*times[i]-4.0*distances[i]).sqrt();
        if delta==0.0{
            res*=0;
            break;
        }
        let x1=(times[i]-delta)/2.0;
        let x2=(times[i]+delta)/2.0;
        let x1_round=x1.ceil();
        let x2_round=x2.floor();
        let mut cur_res = x2_round-x1_round+1.0;
        if x1==x1_round{
            cur_res-=1.0;
        }
        if x2==x2_round{
            cur_res-=1.0;
        }
        res*=cur_res as i32;
    }
    println!("{res}");
    Ok(())
}