use std::fs;

struct PartNumber {
    number : i32,
    row : usize,
    col : usize,
    len : usize
}

struct Gear {
    first_num : i32,
    second_num : i32
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    let lines : Vec<String> = input.split("\n").map(str::to_string).collect();
    let mut schematic: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    let mut part_nums: Vec<Vec<PartNumber>> = Default::default();
    let mut gears: Vec<Gear> = Default::default();

    for i in 0..lines.len() {
        schematic.push(Vec::with_capacity(lines[i].len()));
        part_nums.push(Vec::with_capacity(lines[i].len()));
        for character in lines[i].chars() {
            schematic[i].push(character);
        }
    }

    // Parse for parts
    for i in 0..schematic.len() {
        let mut j: usize = 0;
        while j < schematic[i].len() {
            if schematic[i][j].is_digit(10) {
                let mut current_num: String = schematic[i][j].to_string();
                let mut part: PartNumber = PartNumber { number: 0, row: 0, col: 0, len: 0 };
                part.row = i;
                part.col = j;

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

                if num_adjacent_to_symbol(&schematic, i, part.col, current_num.len()) {
                    part.len = current_num.len();
                    part.number = parsed_num;
                    part_nums[i].push(part);
                } 
            } else {
                j += 1;
            }
        }
    }


    // Parse for gears 
    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            if schematic[i][j] == '*' {
                let mut parts_found: i32 = 0;
                let mut gear: Gear = Gear { first_num: 0, second_num: 0 };
                for k in (i - 1) .. (i + 2) {
                    for part in &part_nums[k] {
                        let r = ((part.col as i32) - 1) .. ((part.col as i32) + (part.len as i32) + 1);
                        if r.contains(&(j as i32)) {
                            parts_found += 1;

                            if parts_found == 1 {
                                gear.first_num = part.number;
                            } else if parts_found == 2 {
                                gear.second_num = part.number;
                            }
                        }
                    }
                }

                if parts_found == 2 {
                    gears.push(gear);
                }
            }
        }
    }

    let mut sum: i32 = 0;

    for gear in gears {
        sum += gear.first_num * gear.second_num;
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
