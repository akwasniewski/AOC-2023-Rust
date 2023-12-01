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
        let mut last_five: String="".to_string();
        for ch in buffer.trim().chars(){
            if last_five.len()>=5{
                last_five.remove(0);
            }
            last_five+=&ch.to_string();
            let mut cur_digit: char='0';
            let mut has_digit=false;
            let len=last_five.len();
            if last_five.len()>=3 && &last_five[(len-3)..]=="one"{
                cur_digit='1';
                has_digit=true;
            }
            else if last_five.len()>=3 && &last_five[(len-3)..]=="two"{
                cur_digit='2';
                has_digit=true;
            }
            else if last_five.len()>=5 && last_five=="three"{
                cur_digit='3';
                has_digit=true;
            }
            else if last_five.len()>=4 && &last_five[(len-4)..]=="four"{
                cur_digit='4';
                has_digit=true;
            }
            else if last_five.len()>=4 && &last_five[(len-4)..]=="five"{
                cur_digit='5';
                has_digit=true;
            }
            else if last_five.len()>=3 && &last_five[(len-3)..]=="six"{
                cur_digit='6';
                has_digit=true;
            }
            else if last_five.len()>=5 && last_five=="seven"{
                cur_digit='7';
                has_digit=true;
            }
            else if last_five.len()>=5 && last_five=="eight"{
                cur_digit='8';
                has_digit=true;  
            }
            else if last_five.len()>=4 && &last_five[(len-4)..]=="nine"{
                cur_digit='9';
                has_digit=true;  
            }
            else if ch.to_digit(10).is_some(){
                cur_digit=ch;
                has_digit=true;
            }
            if has_digit{
                if has_first==false{
                    has_first=true;
                    first=cur_digit;
                    last=cur_digit;
                }
                else{
                    last=cur_digit;
                }
            }
        }
        let res: String = first.to_string()+&last.to_string();
        sum += res.parse::<i32>().unwrap();
    }
    println!("{sum}");
    Ok(())
}