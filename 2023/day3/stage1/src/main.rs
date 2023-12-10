use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    let lines : Vec<String> = input.split("\n").map(str::to_string).collect();
    let mut schematic: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    let mut part_nums: Vec<i32> = Default::default();

    for i in 0..lines.len() {
        schematic.push(Vec::with_capacity(lines[i].len()));
        for character in lines[i].chars() {
            schematic[i].push(character);
        }
    }

    for i in 0..schematic.len() {
        let mut j: usize = 0;
        while j < schematic[i].len() {
            if schematic[i][j].is_digit(10) {
                let mut current_num: String = schematic[i][j].to_string();
                let start = j;

                if j < (schematic[i].len() - 1) {
                    j += 1;
                }

                while schematic[i][j].is_digit(10) && j < schematic[i].len() {
                    current_num += &schematic[i][j].to_string();
                    j += 1;

                    if j >= schematic[i].len() {
                        break;
                    }
                }

                let parsed_num = current_num.parse::<i32>().unwrap();

                if num_adjacent_to_symbol(&schematic, i, start, current_num.len()) {
                    part_nums.push(parsed_num);
                } 
            } else {
                j += 1;
            }
        }
    }

    let mut sum: i32 = 0;

    for part in part_nums {
        sum += part;
    }

    println!("Sum: {sum}");
}

fn num_adjacent_to_symbol(schematic: &Vec<Vec<char>>, row: usize, column: usize, len: usize) -> bool {
    for i in column..column + len {
        // To the left
        if i != 0 {
            if row != 0 {
                if is_symbol(schematic[row - 1][i - 1]) {
                    return true;
                }
            }

            if is_symbol(schematic[row][i - 1]) {
                return true;
            }

            if row != (schematic.len() - 1) {
                if is_symbol(schematic[row + 1][i - 1]) {
                    return true;
                }
            }
        }

        // Above
        if row != 0 {
            if is_symbol(schematic[row - 1][i]) {
                return true;
            }
        }

        // Below
        if row != (schematic.len() - 1) {
            if is_symbol(schematic[row + 1][i]) {
                return true;
            }
        }

        // To the right 
        if i != (schematic[row].len() - 1) {
            if row != 0 {
                if is_symbol(schematic[row - 1][i + 1]) {
                    return true;
                }
            }

            if is_symbol(schematic[row][i + 1]) {
                return true;
            }

            if row != (schematic.len() - 1) {
                if is_symbol(schematic[row + 1][i + 1]) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn is_symbol(character: char) -> bool {
    return !character.is_alphanumeric() && (character != '.');
}
