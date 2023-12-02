use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum: usize = 0;
    'main_loop: for cur_line in 1..=line_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let games = buffer.split(":").last().unwrap();
        let games = games.split(";");
        for game in games{
            let showings = game.split(",");
            for showing in showings{
                let showing = showing.trim().split(" ").collect::<Vec<&str>>();
                let ball_count = showing[0].trim().parse::<i32>().unwrap();
                let color = showing[1].trim();
                if (color=="red" && ball_count>12) || (color=="green" && ball_count>13) || (color=="blue" && ball_count>14){
                        continue 'main_loop;
                }
            }
        }
        sum+=cur_line;
    }
    println!("{sum}");
    Ok(())
}