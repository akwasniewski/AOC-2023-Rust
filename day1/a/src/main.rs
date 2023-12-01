use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut sum: i32=0;
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    for _i in 0..line_count{
        let mut buffer = String::new();
        let mut has_first: bool=false;
        let mut first: char='0';
        let mut last: char='0';
        io::stdin().read_line(&mut buffer)?;
        for ch in buffer.trim().chars(){
            if ch.to_digit(10).is_some(){
                if has_first==false{
                    has_first=true;
                    first=ch;
                    last=ch;
                }
                else{
                    last=ch;
                }
            }
        }
        let res: String = first.to_string()+&last.to_string();
        sum += res.parse::<i32>().unwrap();
    }
    println!("{sum}");
    Ok(())
}