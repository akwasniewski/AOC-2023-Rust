use std::io;
fn is_symbol(ch: char) ->bool{
    if ch=='$' || ch=='#' || ch=='*' || ch=='+' || ch=='@' || ch=='%' || ch=='=' || ch=='/' || ch=='&' || ch=='-'{
        return true;
    }
    false
}
fn reset_word(word_start: &mut i32, cur_num: &mut i32){
    *word_start=-1;
    *cur_num=0;
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
        let mut word_start: i32=-1;
        let mut cur_num: i32=0;
        'row_loop: for i in 0..cur_row.len(){
            if cur_row[i].to_string().parse::<i32>().is_ok(){
                cur_num=cur_num*10+cur_row[i].to_string().parse::<i32>().unwrap();
                if word_start==-1{
                    word_start=i as i32;
                }
            }
            else if word_start!=-1{
                if is_symbol(cur_row[(word_start-1) as usize]) || is_symbol(cur_row[i]){
                    sum+=cur_num;
                    reset_word(&mut word_start, &mut cur_num);
                    continue 'row_loop;
                }
                if !last_row.is_empty(){
                    for i in (word_start-1) as usize..=i{
                        if is_symbol(last_row[i]){
                            sum+=cur_num;
                            reset_word(&mut word_start, &mut cur_num);
                            continue 'row_loop;
                        }
                    }
                }
                if !next_row.is_empty(){
                    for i in (word_start-1) as usize..=i{
                        if is_symbol(next_row[i]){
                            sum+=cur_num;
                            reset_word(&mut word_start, &mut cur_num);
                            continue 'row_loop;
                        }
                    }
                }
                reset_word(&mut word_start, &mut cur_num);
            }
        }
    }
    println!("{sum}");
    Ok(())
}