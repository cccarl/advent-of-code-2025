use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use ordered_float::OrderedFloat;

#[derive(Debug, Copy, Clone)]
enum Coord {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
struct KDNode {
    point: (i64, i64, i64),
    axis: Coord,
    idx_in_vec: usize,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

impl KDNode {
    fn new(value: (i64, i64, i64, usize), axis: Coord) -> Self {
        KDNode {
            point: (value.0, value.1, value.2),
            axis,
            idx_in_vec: value.3,
            left: None,
            right: None,
        }
    }

    // don't do the square root to save cpu cycles
    fn distance_squared(&self, compared: (i64, i64, i64)) -> OrderedFloat<f64> {
        OrderedFloat::from(
            (((self.point.0 - compared.0).pow(2))
                + ((self.point.1 - compared.1).pow(2))
                + ((self.point.2 - compared.2).pow(2))) as f64,
        )
    }
}

// https://adventofcode.com/2025/day/8
pub fn day8(input_reader: BufReader<File>) {
    let mut nodes_vec: Vec<(i64, i64, i64, usize)> = vec![];
    for (i, line_res) in input_reader.lines().enumerate() {
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
            i,
        );
        nodes_vec.push(tuple);
    }

    //dbg!(&nodes_vec);
    let tree = build_kd_tree(nodes_vec.clone(), 0);
    //dbg!("THE TREE:", &tree);

    let mut full_circuit = false;
    let mut shortest_pairs_amount = 10000; // ok this worked with a random ass number LOL, the while wasn't needed
    let mut circuits: Vec<Vec<usize>> = vec![];

    let mut the_final_pair_pt2: [(i64, i64, i64, usize); 2] = [(0, 0, 0, 0), (0, 0, 0, 0)];

    // part 2 wants a circuit for ALL nodes yep
    while !full_circuit {
        shortest_pairs_amount += 1; // keep adding until the circuit is fully made
        println!("New attempt, connections goal: {}", shortest_pairs_amount);

        circuits.clear();
        // make all nodes circuits of 1
        for (i, _) in nodes_vec.iter().enumerate() {
            circuits.push(vec![i]);
        }

        let mut shortest_pairs = BinaryHeap::new();

        //println!("Curr count: {}", connection_count);
        for (i, checking_node) in nodes_vec.iter().enumerate() {
            find_shortest_distances(
                &tree.clone(),
                (checking_node.0, checking_node.1, checking_node.2),
                i,
                &mut shortest_pairs,
                shortest_pairs_amount,
            );
        }

        let shortest_connections = shortest_pairs.clone().into_sorted_vec();
        //println!();
        //println!("SHORTESTs: {:?}", shortest_connections);

        for shortest_pair_data in shortest_connections {
            let node_indexes_shortest = shortest_pair_data.1;

            let mut added = false;
            let mut circs_idx_to_remove: Vec<usize> = vec![];
            let mut values_to_add: HashSet<usize> = HashSet::new();
            for (circ_idx, circ) in circuits.iter().enumerate() {
                if circ.contains(&node_indexes_shortest.0)
                    && !circ.contains(&node_indexes_shortest.1)
                {
                    // println!(
                    //     "Adding: {:?} -> {:?}",
                    //     nodes_vec[node_indexes_shortest.1], circ
                    // );

                    the_final_pair_pt2 = [
                        nodes_vec[node_indexes_shortest.0],
                        nodes_vec[node_indexes_shortest.1],
                    ];

                    for val in circ {
                        values_to_add.insert(*val);
                    }
                    values_to_add.insert(node_indexes_shortest.1);
                    circs_idx_to_remove.push(circ_idx);
                    added = true;
                } else if circ.contains(&node_indexes_shortest.1)
                    && !circ.contains(&node_indexes_shortest.0)
                {
                    // println!(
                    //     "Adding: {:?} -> {:?}",
                    //     nodes_vec[node_indexes_shortest.0], circ
                    // );

                    the_final_pair_pt2 = [
                        nodes_vec[node_indexes_shortest.0],
                        nodes_vec[node_indexes_shortest.1],
                    ];

                    for val in circ {
                        values_to_add.insert(*val);
                    }
                    values_to_add.insert(node_indexes_shortest.0);
                    circs_idx_to_remove.push(circ_idx);
                    added = true;
                } else if circ.contains(&node_indexes_shortest.1)
                    && circ.contains(&node_indexes_shortest.0)
                {
                    // println!(
                    //     "Already added, doing nothing: ({:?})",
                    //     [
                    //         nodes_vec[node_indexes_shortest.0],
                    //         nodes_vec[node_indexes_shortest.1]
                    //     ]
                    // );

                    added = true;
                }
            }

            // add to a new circuit
            if !added {
                // println!(
                //     "Adding: ({:?}, {:?}) -> {:?}",
                //     nodes_vec[node_indexes_shortest.0],
                //     nodes_vec[node_indexes_shortest.1],
                //     circuits
                // );

                circuits.push(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
            } else if !values_to_add.is_empty() {
                // rebuild circuit in case 2 of them connect

                // assuming they are in order...
                for (removed_amount, idx_to_remove) in circs_idx_to_remove.iter().enumerate() {
                    circuits.remove(idx_to_remove - removed_amount);
                }

                the_final_pair_pt2 = [
                    nodes_vec[node_indexes_shortest.0],
                    nodes_vec[node_indexes_shortest.1],
                ];

                let mut new_circuit = vec![];
                for val in values_to_add {
                    new_circuit.push(val);
                }
                circuits.push(new_circuit);
            }
            //println!("Current Circuits: {:?}", circuits);
        }

        //println!("\nFinal: {:?}", circuits);
        if circuits.len() == 1 {
            full_circuit = true;
            println!("FULL CIRCUIT ACHIEVED!: {:?}", circuits);
        } else {
            println!("Failed! curr len: {}", circuits.len());
        }
    }

    // part 1, doesn't work with pt2 since it's only 1 circuit
    // let mut final_mult = 1;
    // circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    // for i in 1..=3 {
    //     let value_highest_len = circuits.get(circuits.len() - i).unwrap().len();
    //     println!("Multing: {}", value_highest_len);
    //     final_mult *= value_highest_len;
    // }
    // println!("Final mult: {}", final_mult);

    println!("Final connection was: {:?}", the_final_pair_pt2);
    println!(
        "Final X mult: {}",
        the_final_pair_pt2[0].0 * the_final_pair_pt2[1].0
    );
}

fn build_kd_tree(mut nodes_vec: Vec<(i64, i64, i64, usize)>, depth: u32) -> Option<Box<KDNode>> {
    if nodes_vec.is_empty() {
        return None;
    }

    let axis = match depth % 3 {
        0 => Coord::X,
        1 => Coord::Y,
        2 => Coord::Z,
        _ => panic!("how"),
    };

    match axis {
        Coord::X => nodes_vec.sort_by(|a, b| a.0.cmp(&b.0)),
        Coord::Y => nodes_vec.sort_by(|a, b| a.1.cmp(&b.1)),
        Coord::Z => nodes_vec.sort_by(|a, b| a.2.cmp(&b.2)),
    }

    let median = nodes_vec.len() / 2;
    let curr_coords = nodes_vec[median];
    let mut new_node = KDNode::new(curr_coords, axis);
    new_node.left = build_kd_tree(nodes_vec[0..median].to_vec(), depth + 1);
    new_node.right = build_kd_tree(nodes_vec[median + 1..nodes_vec.len()].to_vec(), depth + 1);

    Some(Box::new(new_node))
}

fn find_shortest_distances(
    node_opt: &Option<Box<KDNode>>,
    goal_point: (i64, i64, i64),
    idx: usize,
    best_pairs: &mut BinaryHeap<(OrderedFloat<f64>, (usize, usize))>,
    max: usize, // max number of pairs
) {
    if let Some(node) = node_opt {
        
            let index_pair = if node.idx_in_vec < idx {
                (node.idx_in_vec, idx)
            } else {
                (idx, node.idx_in_vec)
            };

            let already_exists = {
                let mut it_do_be_exist = false;
                for pair in best_pairs.iter() {
                    if (pair.1) == index_pair {
                        it_do_be_exist = true;
                        break;
                    }
                }
                it_do_be_exist
            };

            let new_dist_sqrd = node.distance_squared(goal_point);

            if node.idx_in_vec != idx && !already_exists {
                if best_pairs.len() < max {
                    // the bin heap should sort this automatically
                    // dbg!("PROGRESS: ", &best_pairs);
                    // println!("ERM adding: {:?}", index_pair);
                    best_pairs.push((new_dist_sqrd, index_pair));
                } else if new_dist_sqrd < best_pairs.peek().unwrap().0 {
                    // dbg!("PROGRESS: ", &best_pairs);
                    // println!("ERM adding: {:?}", index_pair);
                    // only add if it's actually shorter than the worst
                    best_pairs.pop();
                    best_pairs.push((new_dist_sqrd, index_pair));
                }
            }

            let plane_dist_diff = match node.axis {
                Coord::X => goal_point.0 - node.point.0,
                Coord::Y => goal_point.1 - node.point.1,
                Coord::Z => goal_point.2 - node.point.2,
            };

            let (near, far) = if plane_dist_diff < 0 {
                (&node.left, &node.right)
            } else {
                (&node.right, &node.left)
            };

            find_shortest_distances(near, goal_point, idx, best_pairs, max);
            let worst_dist = best_pairs
                .peek()
                .map(|x| x.0.into_inner())
                .unwrap_or(f64::INFINITY);

            if ((plane_dist_diff * plane_dist_diff) as f64) < worst_dist {
                find_shortest_distances(far, goal_point, idx, best_pairs, max);
            }
    }
}
