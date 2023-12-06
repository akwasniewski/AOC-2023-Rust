use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let times=buffer.trim().split(":").last().unwrap().trim().split_whitespace();
    let mut time="".to_string();
    for i in times{
        time+=i.trim();
    }
    let time=time.parse::<f64>().unwrap();
    buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let distances=buffer.trim().split(":").last().unwrap().trim().split_whitespace();
    let mut distance="".to_string();
    for i in distances{
        distance+=i.trim();
    }
    let distance=distance.parse::<f64>().unwrap();
    let delta=(time*time-4.0*distance).sqrt();
    let x1=(time-delta)/2.0;
    let x2=(time+delta)/2.0;
    let x1_round=x1.ceil();
    let x2_round=x2.floor();
    let mut cur_res = x2_round-x1_round+1.0;
    if x1==x1_round{
        cur_res-=1.0;
    }
    if x2==x2_round{
        cur_res-=1.0;
    }
    println!("{}",cur_res);
    Ok(())
}