use std::fs;

struct Card {
    winning_nums: Vec<i32>,
    nums: Vec<i32>
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    let lines : Vec<String> = input.split("\n").map(str::to_string).collect();
    let mut cards: Vec<Card> = Default::default();

    for line in lines {
       let card_separator: Vec<String> = line.split(":").map(str::to_string).collect();
       let num_string = &card_separator[1];

       let num_separator: Vec<String> = num_string.split("|").map(str::to_string).collect();
       let winning_num_string = &num_separator[0].trim();
       let num_string = &num_separator[1].trim();
       let mut card: Card = Card { winning_nums: Default::default(), nums: Default::default() };

       let winning_nums: Vec<String> = winning_num_string.split(" ").map(str::to_string).collect();
       let nums: Vec<String> = num_string.split(" ").map(str::to_string).collect();

       for num in winning_nums {
            if num != "" {
                card.winning_nums.push(num.parse::<i32>().unwrap());
            }
       }

       for num in nums {
           if num != "" {
                card.nums.push(num.parse::<i32>().unwrap());
            }
       }

        cards.push(card);
    }

    let mut sum: i32 = 0;

    for card in cards {
        let mut winnings: i32 = 0;
        for num in card.nums {
            if card.winning_nums.contains(&num) {
                if winnings == 0 {
                    winnings = 1;
                } else {
                    winnings *= 2;
                }
            }
        }

        sum += winnings;
    }

    println!("Sum: {sum}");
}
