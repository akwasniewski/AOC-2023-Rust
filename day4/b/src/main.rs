use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut sum: usize = 0;
    let mut copy_count = vec![1; line_count];
    for cur_line in 0..line_count{
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
                    correct_guesses+=1;
                    continue 'guess_loop;
                }
            }
        }
        for i in 1..=correct_guesses{
            copy_count[cur_line+i]+=copy_count[cur_line];
        }
    }
    for i in copy_count{
        sum+=i;
    }
    println!("{sum}");
    Ok(())
}