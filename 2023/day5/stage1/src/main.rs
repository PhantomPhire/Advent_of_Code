use std::fs;

struct Mapping
{
    source_start: i64,
    dest_start: i64,
    range_len: i64
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    let sections: Vec<String> = input.split("\n\n").map(str::to_string).collect();
    let seeds_strs: Vec<String> = sections[0].split(" ").map(str::to_string).collect();

    let mut seeds: Vec<i64> = Default::default();
    let mut groups: Vec<Vec<Mapping>> = Default::default();

    for i in 1..seeds_strs.len() {
        seeds.push(seeds_strs[i].parse::<i64>().unwrap());
    } 

    for i in 1..sections.len() {
        let lines: Vec<String> = sections[i].split("\n").map(str::to_string).collect();
        let mut mappings: Vec<Mapping> = Default::default();
        for j in 1..lines.len() {
            let num_strs: Vec<String> = lines[j].split(" ").map(str::to_string).collect();
            
            // Assumes 3 nums
            mappings.push(Mapping { source_start: num_strs[1].parse::<i64>().unwrap(), 
                                    dest_start: num_strs[0].parse::<i64>().unwrap(),
                                    range_len: num_strs[2].parse::<i64>().unwrap() });
        }
        groups.push(mappings);
    }

    let mut locations: Vec<i64> = seeds.clone();

    for i in 0..locations.len() {
        for mappings in &groups {
            for mapping in mappings {
                if (mapping.source_start .. (mapping.source_start + mapping.range_len)).contains(&locations[i]) {
                    locations[i] = locations[i] - mapping.source_start + mapping.dest_start;
                    break;
                }
            }
        }
    }

    let mut lowest: i64 = locations.first().unwrap().to_owned();

    for location in locations {
        if location < lowest {
            lowest = location;
        }
    }

    println!("Lowest: {lowest}");
}
