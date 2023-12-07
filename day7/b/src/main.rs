use std::cmp::Ordering;
#[derive(Eq, PartialEq)]
struct Hand{
    label: Vec<i64>,
    bid_amount: i64,
    rank: i64,
}
impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
            .then_with(|| self.label[0].cmp(&other.label[0]))
            .then_with(|| self.label[1].cmp(&other.label[1]))
            .then_with(|| self.label[2].cmp(&other.label[2]))
            .then_with(|| self.label[3].cmp(&other.label[3]))
            .then_with(|| self.label[4].cmp(&other.label[4]))
    }
}
use std::io;
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let line_count=buffer.trim().parse().unwrap();
    let mut hands: Vec<Hand> = Vec::new();
    for _ in 0..line_count{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut buffer = buffer.trim().split_whitespace();
        let chars: Vec<char> = buffer.next().unwrap().chars().collect();
        let mut sorted_chars: Vec<char>=chars.clone();
        let mut label: Vec<i64>=Vec::new();
        for cur in chars{
            let parsed_char=cur.to_string().parse::<i64>();
            if parsed_char.is_ok(){
                label.push(parsed_char.unwrap())
            }
            else{
                match cur{
                    'A'=> label.push(13),
                    'K'=> label.push(12),
                    'Q'=> label.push(11),
                    'J'=> label.push(1),
                    'T'=> label.push(10),
                    _ => panic!("Failed to parse input {}", cur),
                }
            }
        }
        let mut joker_count: i64=0;
        sorted_chars.sort_by(|a, b| b.cmp(a));
        for i in &sorted_chars{
            if *i=='J'{
                joker_count+=1;
            }
        }
        sorted_chars.retain(|&x|{ x !='J'});
        sorted_chars.push('.');//padding
        let mut highest_count: i64=0;
        let mut cur_count: i64=1;
        let mut rank:Option<i64> = None;
        for i in 1..sorted_chars.len(){
            if sorted_chars[i]==sorted_chars[i-1]{
                cur_count+=1;
            }
            else{
                if highest_count<=1{
                    highest_count=cur_count;
                }
                else if cur_count != 1{
                    if highest_count != cur_count{
                        //full house
                        rank=Some(5);
                    }
                    else{
                        rank=Some(3);
                    }
                }
                cur_count=1;
            }
        }
        highest_count+=joker_count;
        if rank==Some(3) && joker_count==1{
            rank=Some(5);
        }
        if !rank.is_some(){
            match highest_count{
                5 => rank=Some(7),
                4 => rank=Some(6),
                3 => rank=Some(4),
                2 => rank=Some(2),
                1 => rank=Some(1),
                _ => panic!("Failed to parse input {}", highest_count),
            }
        }
        let rank=rank.unwrap();
        let bid_amount=buffer.next().unwrap().parse::<i64>().unwrap();
        hands.push(Hand{label, bid_amount, rank});
    }
    let mut sum=0;
    hands.sort();
    for i in 0..hands.len(){
        sum+=(i as i64+1)*hands[i].bid_amount;
    }
    println!("{sum}");
    Ok(())
}