use std::io;
fn extrapolate(values: &mut Vec<i32>){
    let mut difs: Vec<i32> = Vec::new();
    let mut zeros_count = 0;
    for i in 1..values.len(){
        difs.push(values[i]-values[i-1]);
        if difs[i-1]==0{
            zeros_count+=1;
        }
    }
    if zeros_count==difs.len(){
        difs.push(0);  
    }
    else{
        extrapolate(&mut difs);
        values.push(difs.last().unwrap()+values.last().unwrap());
    }

}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum: i32 = 0;
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut values: Vec<i32>=buffer.trim().split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        extrapolate(&mut values);
        sum+=values.last().unwrap();
    }
    println!("{sum}");
    Ok(())
}