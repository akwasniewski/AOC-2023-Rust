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
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim().split(",");
    let mut sum = 0;
    for i in buffer{
        sum+=hash(i.trim());
    }
    println!("{sum}");
    Ok(())
}