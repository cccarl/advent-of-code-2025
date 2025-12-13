use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/7
pub fn day7(input_reader: BufReader<File>) {
    let mut lazersss: HashSet<usize> = HashSet::new();
    let mut lazers_pt2: HashMap<usize, i64> = HashMap::new();
    let mut split_times = 0;
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut splitters: Vec<usize> = vec![];
        for (i, char) in line.chars().enumerate() {
            if char == 'S' {
                lazersss.insert(i);
                lazers_pt2.insert(i, 1);
            } else if char == '^' {
                splitters.push(i);
            }
        }

        let mut new_lazers: HashSet<usize> = HashSet::new();
        let mut new_lazers_but_keep_repeats: HashMap<usize, i64> = HashMap::new();
        for pew in &lazersss {
            if splitters.contains(pew) {
                new_lazers.insert(*pew + 1);
                new_lazers.insert(*pew - 1);
                split_times += 1;
            } else {
                new_lazers.insert(*pew);
            }
        }
        for pew2 in &lazers_pt2 {
            if splitters.contains(pew2.0) {
                let prev_pos = new_lazers_but_keep_repeats.get(&(pew2.0 - 1));
                match prev_pos {
                    Some(val) => new_lazers_but_keep_repeats.insert(*pew2.0 - 1, val + *pew2.1),
                    None => new_lazers_but_keep_repeats.insert(*pew2.0 - 1, *pew2.1),
                };

                let next_pos = new_lazers_but_keep_repeats.get(&(pew2.0 + 1));
                match next_pos {
                    Some(val) => new_lazers_but_keep_repeats.insert(*pew2.0 + 1, val + *pew2.1),
                    None => new_lazers_but_keep_repeats.insert(*pew2.0 + 1, *pew2.1),
                };
            } else {
                let pos = new_lazers_but_keep_repeats.get(pew2.0);
                match pos {
                    Some(val) => new_lazers_but_keep_repeats.insert(*pew2.0, val + *pew2.1),
                    None => new_lazers_but_keep_repeats.insert(*pew2.0, *pew2.1),
                };
            }
        }
        println!("{:?}", new_lazers_but_keep_repeats);

        lazersss = new_lazers;
        lazers_pt2 = new_lazers_but_keep_repeats;
    }

    println!("{:?}", lazersss);
    println!(
        "[Part 1] The lazer was split this many times: {}",
        split_times
    );
    let mut final_pt2_sum = 0;
    for (_k, v) in lazers_pt2 {
        final_pt2_sum += v;
    }
    println!(
        "[Part 2] The lazer amount in all worlds is: {}",
        final_pt2_sum
    );
}
