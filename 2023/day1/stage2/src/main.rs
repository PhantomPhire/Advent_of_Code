use std::fs;

static NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    
    let lines : Vec<String> = input.split("\n").map(str::to_string).collect();

    let mut sum : i32 = 0;
    
    for line in lines {
        let line_sanitized: String = line.to_lowercase();
        let mut first: char = '\0';
        let mut first_i: i32 = -1;
        let mut last: char = '\0';
        let mut last_i: i32 = -1;
        let mut i: i32 = 0;

        for character in line_sanitized.chars() {
            if character.is_digit(10) {
                if first == '\0' {
                    first = character;
                    first_i = i;
                } 
                last = character;
                last_i = i;
           }

            i += 1;
        }

        i = 1;
        for number_string in NUMBERS {
            let matches: Vec<_> = line_sanitized.match_indices(number_string).collect(); 
            for single_match in matches {
                let v = single_match.0;
                if ((v as i32) < first_i) || (first_i == -1) {
                    first_i = v as i32;
                    first = ((i + 0x30) as u8) as char;
                }
                if (v as i32) > last_i {
                    last_i = v as i32;
                    last = ((i + 0x30) as u8) as char;
                }
            }
            i += 1;
        }
        if first != '\0' {
            let concat_string : String = first.to_string() + &last.to_string();
            let parsed = (concat_string).parse::<i32>().unwrap();
            sum += parsed;
        } 
    }

    println!("Sum: {sum}");
}
