use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut buffer = buffer.trim().split_whitespace();
    let row_count = buffer.next().unwrap().parse().unwrap();
    let row_length = buffer.next().unwrap().parse().unwrap();
    let mut empty_spaces=vec![0; row_length];
    let mut sum=0;
    for row in 0..row_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer: Vec<char> = buffer.chars().collect();
        for i in 0..row_length{
            if buffer[i]=='O'{
                if empty_spaces[i]!=0{
                    sum+=row_count-row+empty_spaces[i];
                }
                else{
                    sum+=row_count-row;
                }
            }
            else if buffer[i]=='#'{
                empty_spaces[i]=0;
            }
            else{
                empty_spaces[i]+=1;
            }
        }
    }
    println!("\n{sum}");
    Ok(())
}