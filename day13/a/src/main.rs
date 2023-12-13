use std::io;
fn analyze(cur_map: Vec<String>) -> Option<usize>{
    'main_loop: for i in 1..cur_map.len(){
        let mut cur_left: i32=i as i32 -1;
        let mut cur_right: i32=i as i32;
        while cur_left>=0 && (cur_right as usize)<cur_map.len(){
            if cur_map[cur_left as usize]!=cur_map[cur_right as usize]{
                continue 'main_loop;
            }
            cur_left-=1;
            cur_right+=1;
        }
        return Some(i);
    }
    return None;
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut row_map: Vec<String> = Vec::new();
    let mut column_map: Vec<String> = Vec::new();
    let mut sum_rows = 0;
    let mut sum_columns = 0;
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.len()==1{
            let res = analyze(row_map.clone());
            if res.is_some(){
                sum_rows+=res.unwrap();
            }
            let res = analyze(column_map.clone());
            if res.is_some(){
                sum_columns+=res.unwrap();
            }
            row_map=Vec::new();
            column_map=Vec::new();
            continue;
        }
        row_map.push(buffer.trim().to_owned());
        if column_map.len()==0{
            column_map.resize(buffer.len()-1, "".to_string());
        }
        let buf_chars=buffer.chars().collect::<Vec<char>>();
        for i in 0..buffer.len()-1{
            column_map[i].push_str(&buf_chars[i].to_string())
        }
    }
    let res = analyze(row_map.clone());
    if res.is_some(){
        sum_rows+=res.unwrap();
    }
    let res = analyze(column_map.clone());
    if res.is_some(){
        sum_columns+=res.unwrap();
    }
    println!("{sum_rows} {sum_columns}");
    let res = sum_rows*100+sum_columns;
    println!("res: {res}");
    Ok(())
}