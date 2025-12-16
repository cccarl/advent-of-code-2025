use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/7
pub fn day8(input_reader: BufReader<File>) {
    let mut nodes: Vec<(i64, i64, i64)> = vec![];
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let line_nums = line.split(',');
        let mut line_numes_parsed =
            line_nums.map(|num_str| num_str.parse::<i64>().expect("COULD not parse into u64"));

        let tuple = (
            line_numes_parsed.next().expect("1st elem failed"),
            line_numes_parsed.next().expect("2nd elem failed"),
            line_numes_parsed.next().expect("3rd elem failed"),
        );
        nodes.push(tuple);
    }

    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut all_connections_made: HashSet<Vec<usize>> = HashSet::new();
    let mut connection_count = 0;
    while connection_count < 10 {
        println!("");
        let mut shortest_distance: Option<f64> = None;
        let mut node_indexes_shortest: (usize, usize) = (0, 0);
        for (i, checking_node) in nodes.iter().enumerate() {
            for (j, checked_against_node) in nodes.iter().enumerate() {
                if i == j {
                    continue;
                }

                let mut already_added = false;
                for conn in &all_connections_made {
                    if conn.contains(&i) && conn.contains(&j) {
                        println!("Skipping: {}-{} in {:?}", i, j, conn);
                        already_added = true;
                        break;
                    }
                }
                if already_added {
                    continue;
                }

                let distance: f64 = (((checking_node.0 - checked_against_node.0).pow(2)
                    + (checking_node.1 - checked_against_node.1).pow(2)
                    + (checking_node.2 - checked_against_node.2).pow(2))
                    as f64)
                    .sqrt();

                if shortest_distance.is_none() || shortest_distance.unwrap() > distance {
                    println!("New shortest distance: {} {}", i, j);
                    shortest_distance = Some(distance);
                    node_indexes_shortest = (i, j);
                }
            }
        }

        let mut added = false;
        for circ in &mut circuits {
            if circ.contains(&node_indexes_shortest.0) && !circ.contains(&node_indexes_shortest.1) {
                println!("Adding: {} -> {:?}", node_indexes_shortest.1, circ);
                all_connections_made.insert(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
                circ.push(node_indexes_shortest.1);
                connection_count += 1;
                added = true;
            } else if circ.contains(&node_indexes_shortest.1)
                && !circ.contains(&node_indexes_shortest.0)
            {
                println!("Adding: {} -> {:?}", node_indexes_shortest.0, circ);
                all_connections_made.insert(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
                circ.push(node_indexes_shortest.0);
                connection_count += 1;
                added = true;
            } else if circ.contains(&node_indexes_shortest.1)
                && circ.contains(&node_indexes_shortest.0)
            {
                println!(
                    "Already added, doing nothing: ({}, {})",
                    node_indexes_shortest.0, node_indexes_shortest.1
                );
                all_connections_made.insert(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
                added = true;
            }
        }

        if !added {
            println!(
                "Adding: ({}, {}) -> {:?}",
                node_indexes_shortest.0, node_indexes_shortest.1, circuits
            );
            all_connections_made.insert(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
            circuits.push(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
            connection_count += 1;
        }
        println!("Current Circuits: {:?}", circuits);
    }

    println!("\nFinal: {:?}", circuits);
}
