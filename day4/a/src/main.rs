use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum: usize = 0;
    for _ in 0..line_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let games = buffer.split(":").last().unwrap();
        let mut games = games.split("|");
        let winners: Vec<&str> = games.next().unwrap().split(" ").collect();
        let guesses: Vec<&str> = games.next().unwrap().split(" ").collect();
        let mut correct_guesses=0;
        'guess_loop: for guess in guesses{
            for winner in &winners{
                if guess.trim()==winner.trim() && guess.trim()!=""{
                    if correct_guesses==0{
                        correct_guesses=1;
                    }
                    else{
                        correct_guesses*=2;
                    }
                    continue 'guess_loop;
                }
            }
        }
        sum+=correct_guesses;
    }
    println!("{sum}");
    Ok(())
}