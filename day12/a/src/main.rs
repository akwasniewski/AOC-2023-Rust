use std::io;
fn analyze(springs: &mut Vec<char>, groups: &Vec<usize>) -> usize{
    let mut cur_group=0;
    let mut group_count=0;
    for i in 0..springs.len(){
        if springs[i]=='#'{
            group_count+=1;
            if cur_group>=groups.len() || group_count>groups[cur_group]{
                return 0;
            }
        }
        else if springs[i]=='.'{
            if group_count != 0 && group_count != groups[cur_group]{
                return 0;
            }
            else if group_count !=0{
                group_count=0;
                cur_group+=1;
            }
        }
        else{
            let mut sum: usize = 0;
            springs[i]='.';
            sum+=analyze(springs, &groups);
            springs[i]='#';
            sum+=analyze(springs, &groups);
            springs[i]='?';
            return sum;
        }
    }
    if cur_group==groups.len(){
        return 1;
    }
    return 0;
}
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum=0;
    for _ in 0..line_count{
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut buffer=buffer.trim().split_whitespace();
        let mut springs: Vec<char>=buffer.next().unwrap().chars().collect();
        springs.push('.');
        let groups: Vec<usize>=buffer.next().unwrap().trim().split(',').map(|x| x.parse().unwrap()).collect();
        let res = analyze(&mut springs, &groups);
        sum+=res;
    }
    println!("{sum}");
    Ok(())
}