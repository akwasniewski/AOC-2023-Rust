use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum: usize = 0;
    for cur_line in 1..=line_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let games = buffer.split(":").last().unwrap();
        let games = games.split(";");
        let mut max_red: usize=0;
        let mut max_green: usize=0;
        let mut max_blue: usize=0;
        for game in games{
            let showings = game.split(",");
            for showing in showings{
                let showing = showing.trim().split(" ").collect::<Vec<&str>>();
                let ball_count = showing[0].trim().parse::<usize>().unwrap();
                let color = showing[1].trim();
                if color=="red" && ball_count>max_red{
                    max_red=ball_count;
                }                
                else if color=="green" && ball_count>max_green{
                    max_green=ball_count;
                }                
                else if color=="blue" && ball_count>max_blue{
                    max_blue=ball_count;
                }
            }
        }
        sum+=max_red*max_green*max_blue;
    }
    println!("{sum}");
    Ok(())
}