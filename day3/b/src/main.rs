use std::io;
fn is_number(ch: char)->bool{
    return ch.to_string().parse::<i32>().is_ok();
}
fn find_number(row: &Vec<char>, mut i: usize)->i32{
    while is_number(row[i-1]){i-=1;};
    let mut res: i32=0;
    while is_number(row[i]){
        res=10*res+row[i].to_string().parse::<i32>().unwrap();
        i+=1;
    }
    return res;
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count:usize=buffer.trim().parse().unwrap();
    let mut sum: i32 = 0;
    let mut last_row: Vec<char>;
    let mut cur_row: Vec<char> = Vec::new();
    let mut next_row: Vec<char> = Vec::new();
    for cur_line in 0..=line_count{
        last_row=cur_row;
        cur_row=next_row;
        if cur_line==line_count{
            next_row=Vec::new();
        }
        else{
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            next_row=buffer.chars().collect();
            next_row.push('.');//for ease of use
            next_row.insert(0, '.');
        }
        if cur_row.is_empty(){continue};
        for i in 1..cur_row.len()-1{
            let mut numbers:i32=0;
            let mut mult: i32=1;
            if cur_row[i]=='*'{
                if is_number(cur_row[i-1]){
                    numbers+=1;
                    mult*=find_number(&cur_row, i-1);
                } 
                if is_number(cur_row[i+1]){
                    numbers+=1;
                    mult*=find_number(&cur_row, i+1);
                }
                if !last_row.is_empty(){
                    if is_number(last_row[i-1]){
                        numbers+=1;
                        mult*=find_number(&last_row, i-1);
                        if !is_number(last_row[i]) && is_number(last_row[i+1]){
                            numbers+=1;
                            mult*=find_number(&last_row, i+1);
                        }
                    }
                    else if is_number(last_row[i]){
                        numbers+=1;
                        mult*=find_number(&last_row, i);
                    }
                    else if is_number(last_row[i+1]){
                        numbers+=1;
                        mult*=find_number(&last_row, i+1);
                    }
                }
                if !next_row.is_empty(){
                    if is_number(next_row[i-1]){
                        numbers+=1;
                        mult*=find_number(&next_row, i-1);
                        if !is_number(next_row[i]) && is_number(next_row[i+1]){
                            numbers+=1;
                            mult*=find_number(&next_row, i+1);
                        }
                    }
                    else if is_number(next_row[i]){
                        mult*=find_number(&next_row, i);
                        numbers+=1;
                    }
                    else if is_number(next_row[i+1]){
                        mult*=find_number(&next_row, i+1);
                        numbers+=1;
                    }
                }
            }
            if numbers==2{
                sum+=mult;
            }
        }
    }
    println!("{sum}");
    Ok(())
}