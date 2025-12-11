use std::{
    collections::HashSet, fs::File, io::{BufRead, BufReader}
};

enum CurrentSection {
    Ranges,
    Values,
}

// https://adventofcode.com/2025/day/3
pub fn day5(input_reader: BufReader<File>) {
    let mut section = CurrentSection::Ranges;
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut final_fresh_value_count = 0;
    let mut all_ranges_no_dupes:HashSet<(u64, u64)> = HashSet::new();

    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        match section {
            CurrentSection::Ranges => {
                if line.is_empty() {
                    //println!("Next section reached!");
                    section = CurrentSection::Values;

                    // part 2 -> remove any overlapping range
                    // the approach is with a double for, it compares the values within itself
                    // it removes ranges included in others, and expand ranges that are partially included in others
                    // it will repeat the process until no range was modified
                    // main downside is ending with duplicate ranges, which is fixed by transferring the values to a hashset after this
                    let mut is_fixed: bool;
                    let mut initial_vec = ranges.clone();
                    let mut fixed_overlaps_vec: Vec<(u64, u64)> = vec![];
                    loop {
                        fixed_overlaps_vec.clear();
                        is_fixed = true;
                        for range_to_check in &initial_vec {
                            //println!("\nCHECKING: {:?}", range_to_check);
                            let mut value_has_no_overlap = true;
                            for range_from_total in &initial_vec {

                                // don't check yourself!
                                if range_to_check == range_from_total {
                                    continue;
                                }

                                let fixed_range = 
                                // range is overlapped by other one, don't include in new vec
                                if range_from_total.0 <= range_to_check.0 && range_to_check.1 <= range_from_total.1 {
                                    value_has_no_overlap = false;
                                    break;
                                } 
                                // range is bigger than other one, include directly, prev case removes the smaller one
                                else if range_to_check.0 <= range_from_total.0 &&  range_from_total.1 <= range_to_check.1 {
                                    value_has_no_overlap = false;
                                    range_to_check.clone()
                                }
                                // range start is included in another range, expand and save
                                else if range_from_total.0 <= range_to_check.0 && range_to_check.0 <= range_from_total.1 {
                                    value_has_no_overlap = false;
                                    (range_from_total.0, range_to_check.1)
                                }
                                // range end is included in another range, expand and save
                                else if range_from_total.0 <= range_to_check.1 && range_to_check.1 <= range_from_total.1 {
                                    value_has_no_overlap = false;
                                    (range_to_check.0, range_from_total.1)
                                }
                                // no overlap at all... for now
                                else {
                                    continue;
                                };

                                println!("Overlap detected: {:?} {:?}", range_from_total, range_to_check);
                                println!("Adding fixed version: {:?}", fixed_range);
                                is_fixed = false;
                                fixed_overlaps_vec.push(fixed_range);
                                break;
                            }
                            
                            
                            if value_has_no_overlap {
                                //println!("Adding, no overlaps: {:?}", range_to_check);
                                fixed_overlaps_vec.push(range_to_check.clone());
                            }

                            //println!("Fixed progress: {:?}", fixed_overlaps_vec);
                            
                        }

                        if is_fixed {
                            println!("Finished! no changes detected\n");
                            break;
                        } else {
                            println!("\nRepeating!");
                            initial_vec = fixed_overlaps_vec.clone();
                        }
                    }
                    //println!("Final ranges: {:?}", fixed_overlaps_vec);
                    
                    for range in fixed_overlaps_vec {
                        all_ranges_no_dupes.insert(range);
                    }
                    println!("Final ranges set: {:?}", all_ranges_no_dupes);

                    println!("Calculating number of fresh ids...\n");
                    let mut amount_of_fresh_ids = 0;
                    for range in all_ranges_no_dupes.iter() {
                        let range_size = range.1 - range.0 + 1;
                        amount_of_fresh_ids += range_size;
                    }

                    println!("[Part 2] Amount of fresh ids: {}", amount_of_fresh_ids);
                    continue;
                }
                let mut range_split = line.split('-');
                let range_start: u64 = range_split
                    .next()
                    .expect("This range start is not here!")
                    .parse()
                    .expect("NUMBER???");
                let range_end: u64 = range_split
                    .next()
                    .expect("This range end is not here!")
                    .parse()
                    .expect("NUMBER???");
                ranges.push((range_start, range_end));
            }
            CurrentSection::Values => {
                // part 1
                let value: u64 = line
                    .parse()
                    .expect("VALUE in line could not be a number wtf");
                let mut spoiled_value = true;
                for range in &ranges {
                    if range.0 <= value && value <= range.1 {
                        spoiled_value = false;
                    }
                }
                if !spoiled_value {
                    final_fresh_value_count += 1;
                }
            }
        }
    }

    println!(
        "[Part 1] The amount of fresh values is: {}",
        final_fresh_value_count
    );
}
