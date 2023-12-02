use std::fs;
use regex::Regex;

struct BlockCollection {
   red_blocks: i32,
   green_blocks: i32,
   blue_blocks: i32
}

struct BlockGame {
    id: i32,
    rounds: Vec<BlockCollection>
}

static BLOCKS: BlockCollection = BlockCollection {
    red_blocks: 12,
    green_blocks: 13,
    blue_blocks: 14
};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    let line_regex = Regex::new("Game (\\d+):(.+)").unwrap();
    let round_regex = Regex::new("(\\d+) (\\w+)").unwrap();
    
    let lines : Vec<String> = input.split("\n").map(str::to_string).collect();
    let mut possible_games: Vec<BlockGame> = Vec::with_capacity(100);

    for line in lines {
        let Some(caps) = line_regex.captures(&line) else {
            println!("Found bad line in input!");
            continue;
        };
        let mut game = BlockGame{
            id: caps[1].parse::<i32>().unwrap(),
            rounds: Vec::with_capacity(10)
        };
        let round_string: String = caps[2].to_string();

        let rounds: Vec<String> = round_string.split(";").map(str::to_string).collect();
    
        for round in rounds {
            let block_strings: Vec<String> = round.split(",").map(str::to_string).collect();
            let mut blocks = BlockCollection {
                red_blocks: 0,
                green_blocks: 0,
                blue_blocks: 0
            };

            for block_string in block_strings {
                let Some(block_caps) = round_regex.captures(&block_string) else {
                    println!("Found bad block in input!");
                    continue;
                };

                if block_caps[2].contains("red") {
                    blocks.red_blocks = block_caps[1].parse::<i32>().unwrap();
                } else if block_caps[2].contains("green") {
                    blocks.green_blocks = block_caps[1].parse::<i32>().unwrap();
                } else if block_caps[2].contains("blue") {
                    blocks.blue_blocks = block_caps[1].parse::<i32>().unwrap();
                }
            }

            game.rounds.push(blocks);
        }

        let mut possible: bool = true;
        for round in &game.rounds {
            if (round.red_blocks > BLOCKS.red_blocks) ||
               (round.green_blocks > BLOCKS.green_blocks) ||
               (round.blue_blocks > BLOCKS.blue_blocks) {
                possible = false;
                break;
            }
        }

        if possible {
            possible_games.push(game);
        }
    }

    let mut sum: i32 = 0;

    for game in possible_games {
        sum += game.id;
    }

    println!("Sum: {sum}");
}
