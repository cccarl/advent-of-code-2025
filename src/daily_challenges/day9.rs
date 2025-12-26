use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug)]
struct ValidProofSpaces {
    checked_coord: (i64, i64),
    up_left: Option<(i64, i64)>,
    up_right: Option<(i64, i64)>,
    down_left: Option<(i64, i64)>,
    down_right: Option<(i64, i64)>,
}

// https://adventofcode.com/2025/day/9
pub fn day9(input_reader: BufReader<File>) {
    let mut coords: Vec<(i64, i64)> = Vec::new();
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut line_split = line.split(',');
        coords.push((
            line_split
                .next()
                .expect("WRONG char 1")
                .parse()
                .expect("Could not make char 1 an int"),
            line_split
                .next()
                .expect("WRONG char 2")
                .parse()
                .expect("Could not make char 2 an int"),
        ));
    }
    println!("coords: {:?}", coords);

    let mut biggest_area = 0;
    for (i, i_coord) in coords.iter().enumerate() {
        for (j, j_coord) in coords.iter().enumerate() {
            // avoid comparing the same space and repeats
            if i >= j {
                continue;
            }

            let curr_area = (i_coord.0 - j_coord.0 + 1) * (i_coord.1 - j_coord.1 + 1);
            if curr_area > biggest_area {
                biggest_area = curr_area;
            }
        }
    }
    println!("[Part 1] Biggest area: {}", biggest_area);

    let mut invalid_spaces_collection: HashSet<(i64, i64)> = HashSet::new();
    let mut valid_spaces_collection: HashSet<(i64, i64)> = HashSet::new();
    let mut biggest_area_2 = 0;
    for (i, i_coord) in coords.iter().enumerate() {
        for (j, j_coord) in coords.iter().enumerate() {
            // avoid comparing the same space and repeats
            if i >= j {
                continue;
            }

            //println!("Checking area: {:?} - {:?}", i_coord, j_coord);

            // if it's a smaller area just skip that slow thing
            let curr_area =
                ((i_coord.0 - j_coord.0).abs() + 1) * ((i_coord.1 - j_coord.1).abs() + 1);
            if curr_area < biggest_area_2 {
                continue;
            }

            let mut valid_rect = true;
            // iterate over the inner rectangle's perimeter, that should be enough to know if it's a valid rectangle
            let corners_to_check = if i_coord.0 < j_coord.0 {
                if i_coord.1 < j_coord.1 {
                    vec![
                        (i_coord.0 + 1, j_coord.1 - 1),
                        (j_coord.0 - 1, i_coord.1 + 1),
                        (j_coord.0 - 1, i_coord.1 + 1),
                    ]
                } else {
                    vec![
                        (i_coord.0 + 1, j_coord.1 + 1),
                        (j_coord.0 - 1, i_coord.1 - 1),
                    ]
                }
            } else {
                if i_coord.1 < j_coord.1 {
                    vec![
                        (i_coord.0 - 1, j_coord.1 - 1),
                        (j_coord.0 + 1, i_coord.1 + 1),
                    ]
                } else {
                    vec![
                        (i_coord.0 - 1, j_coord.1 + 1),
                        (j_coord.0 + 1, i_coord.1 - 1),
                    ]
                }
            };

            for checking_coord in corners_to_check {
                //println!("Is valid? {:?}", checking_coord);
                // already cached! don't do the check
                if invalid_spaces_collection.contains(&checking_coord) {
                    //println!("Invalid from cache! {:?}", checking_coord);
                    valid_rect = false;
                    break;
                }
                // already known space is valid!
                if valid_spaces_collection.contains(&checking_coord) {
                    continue;
                }
                // check if there are coords in the 4 corners of this space, if there's 1+ missing, not a valid rect
                let mut valid_proof = ValidProofSpaces {
                    checked_coord: checking_coord,
                    ..Default::default()
                };

                // now find the 4 proofs that it's valid, if 1 is None, the entire rectangle is invalid
                for proof_coord in &coords {
                    // already done so just break;
                    if valid_proof.down_left.is_some()
                        && valid_proof.down_right.is_some()
                        && valid_proof.up_left.is_some()
                        && valid_proof.up_right.is_some()
                    {
                        valid_spaces_collection.insert(checking_coord);
                        break;
                    }

                    if valid_proof.up_left.is_none()
                        && proof_coord.0 <= valid_proof.checked_coord.0
                        && proof_coord.1 <= valid_proof.checked_coord.1
                    {
                        valid_proof.up_left = Some(*proof_coord);
                        continue;
                    }
                    if valid_proof.up_right.is_none()
                        && proof_coord.0 >= valid_proof.checked_coord.0
                        && proof_coord.1 <= valid_proof.checked_coord.1
                    {
                        valid_proof.up_right = Some(*proof_coord);
                        continue;
                    }
                    if valid_proof.down_left.is_none()
                        && proof_coord.0 <= valid_proof.checked_coord.0
                        && proof_coord.1 >= valid_proof.checked_coord.1
                    {
                        valid_proof.down_left = Some(*proof_coord);
                        continue;
                    }
                    if valid_proof.down_right.is_none()
                        && proof_coord.0 >= valid_proof.checked_coord.0
                        && proof_coord.1 >= valid_proof.checked_coord.1
                    {
                        valid_proof.down_right = Some(*proof_coord);
                        continue;
                    }
                }
                if valid_proof.down_left.is_none()
                    || valid_proof.down_right.is_none()
                    || valid_proof.up_left.is_none()
                    || valid_proof.up_right.is_none()
                {
                    println!("Invalid spot found! {:?}", valid_proof);
                    invalid_spaces_collection.insert(checking_coord);
                    valid_rect = false;
                    break;
                }
            }

            if !valid_rect {
                continue;
            }

            if curr_area > biggest_area_2 {
                biggest_area_2 = curr_area;
                println!(
                    "New biggest area: {}, from corners: {:?}, {:?}",
                    curr_area, i_coord, j_coord
                );
            }
        }
    }
    println!("[Part 2] Biggest area: {}", biggest_area_2);
}
