use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    
    let lines : Vec<String> = input.split("\n").map(str::to_string).collect();

    let mut sum : i32 = 0;
    
    for line in lines {
        let mut first : char = '\0';
        let mut last  : char = '\0';

        for character in line.chars() {
            if character.is_digit(10) {
                if first == '\0' {
                    first = character;
                } 
                last = character;
           }
        }
        if first != '\0' {

            let concat_string : String = first.to_string() + &last.to_string();
            let parsed = (concat_string).parse::<i32>().unwrap(); 
            sum += parsed;
        } 
    }

    println!("Sum: {sum}");
}
