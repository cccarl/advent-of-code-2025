use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

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
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

impl KDNode {
    fn new(value: (i64, i64, i64), axis: Coord) -> Self {
        KDNode {
            point: value,
            axis,
            left: None,
            right: None,
        }
    }

    fn distance(&self, compared: (i64, i64, i64)) -> f64 {
        ((((self.point.0 - compared.0).pow(2))
            + ((self.point.1 - compared.1).pow(2))
            + ((self.point.2 - compared.2).pow(2))) as f64)
            .sqrt()
    }
}

// https://adventofcode.com/2025/day/8
pub fn day8(input_reader: BufReader<File>) {
    let mut nodes_vec: Vec<(i64, i64, i64)> = vec![];
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
        nodes_vec.push(tuple);
    }

    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut all_connections_made: HashSet<[(i64, i64, i64); 2]> = HashSet::new();
    let mut connection_count = 0;
    while connection_count < 9 {
        println!("Curr count: {}", connection_count);
        let mut node_indexes_shortest: (usize, usize) = (0, 0);
        let mut shortest_distance: Option<f64> = None;
        for (i, checking_node) in nodes_vec.iter().enumerate() {
            // for (j, checked_against_node) in nodes_vec.iter().enumerate() {
            //     if i == j {
            //         continue;
            //     }

            //     let mut already_added = false;
            //     for conn in &all_connections_made {
            //         if conn.contains(&i) && conn.contains(&j) {
            //             //println!("Skipping: {}-{} in {:?}", i, j, conn);
            //             already_added = true;
            //             break;
            //         }
            //     }
            //     if already_added {
            //         continue;
            //     }

            //     let distance: f64 = (((checking_node.0 - checked_against_node.0).pow(2)
            //         + (checking_node.1 - checked_against_node.1).pow(2)
            //         + (checking_node.2 - checked_against_node.2).pow(2))
            //         as f64)
            //         .sqrt();

            //     if shortest_distance.is_none() || shortest_distance.unwrap() > distance {
            //         //println!("New shortest distance: {} {} -> {}", i, j, distance);
            //         shortest_distance = Some(distance);
            //         node_indexes_shortest = (i, j);
            //     }s
            
            // build vec into a kd tree, without the node that's gonna be checked
            let mut filtered_nodes = nodes_vec.clone();
            filtered_nodes.remove(i);
            let tree = build_kd_tree(filtered_nodes, 0);
            
            // get closest using the kd tree
            let closest_node = calculate_closest_node(&tree.clone(), *checking_node, *tree.clone().unwrap()).unwrap();
            
            let curr_distance = calc_distance(closest_node.point, *checking_node);
            if shortest_distance.is_none() || shortest_distance.unwrap() > curr_distance && curr_distance != 0. {
                shortest_distance = Some(curr_distance);
                println!("NEW shortest distance: {:?}", closest_node.distance(*checking_node));
                let index_shortest_found = nodes_vec.iter().position(|node| 
                    node.0 == closest_node.point.0 && node.1 ==  closest_node.point.1 && node.2 == closest_node.point.2
                );
                node_indexes_shortest = (i, index_shortest_found.unwrap());
                println!("NODE INDEES SHORTEST: {:?}", node_indexes_shortest);
            }
        }
        
        

        let mut added = false;
        let mut circs_idx_to_remove: Vec<usize> = vec![];
        let mut values_to_add: HashSet<usize> = HashSet::new();
        for (circ_idx, circ) in circuits.iter().enumerate() {
            if circ.contains(&node_indexes_shortest.0) && !circ.contains(&node_indexes_shortest.1) {
                println!("Adding: {:?} -> {:?}", nodes_vec[node_indexes_shortest.1], circ);
                all_connections_made.insert([nodes_vec[node_indexes_shortest.0], nodes_vec[node_indexes_shortest.1]]);
                for val in circ {
                    values_to_add.insert(*val);
                }
                values_to_add.insert(node_indexes_shortest.1);
                circs_idx_to_remove.push(circ_idx);
                added = true;
            } else if circ.contains(&node_indexes_shortest.1)
                && !circ.contains(&node_indexes_shortest.0)
            {
                println!("Adding: {:?} -> {:?}", nodes_vec[node_indexes_shortest.0], circ);
                all_connections_made.insert([nodes_vec[node_indexes_shortest.0], nodes_vec[node_indexes_shortest.1]]);
                for val in circ {
                    values_to_add.insert(*val);
                }
                values_to_add.insert(node_indexes_shortest.0);
                circs_idx_to_remove.push(circ_idx);
                added = true;
            } else if circ.contains(&node_indexes_shortest.1)
                && circ.contains(&node_indexes_shortest.0)
            {
                println!(
                    "Already added, doing nothing: ({:?})",
                    [nodes_vec[node_indexes_shortest.0], nodes_vec[node_indexes_shortest.1]]
                );
                all_connections_made.insert([nodes_vec[node_indexes_shortest.0], nodes_vec[node_indexes_shortest.1]]);
                added = true;
            }
        }

        // add to a new circuit
        if !added {
            println!(
                "Adding: ({:?}, {:?}) -> {:?}",
                nodes_vec[node_indexes_shortest.0], nodes_vec[node_indexes_shortest.1], circuits
            );
            all_connections_made.insert([nodes_vec[node_indexes_shortest.0], nodes_vec[node_indexes_shortest.1]]);
            circuits.push(vec![node_indexes_shortest.0, node_indexes_shortest.1]);
            connection_count += 1;
        } else if !values_to_add.is_empty() {
            // rebuild circuit in case 2 of them connect
            connection_count += 1;

            // assuming they are in order...
            let mut removed_amount = 0;
            for idx_to_remove in circs_idx_to_remove {
                circuits.remove(idx_to_remove - removed_amount);
                removed_amount += 1;
            }

            let mut new_circuit = vec![];
            for val in values_to_add {
                new_circuit.push(val);
            }
            circuits.push(new_circuit);
        }
        println!("Current Circuits: {:?}", circuits);
    }

    println!("\nFinal: {:?}", circuits);

    let mut final_mult = 1;
    for i in 1..=3 {
        final_mult *= circuits.get(circuits.len() - i).unwrap().len();
    }

    println!("Final mult: {}", final_mult);
}

fn build_kd_tree(mut nodes_vec: Vec<(i64, i64, i64)>, depth: u32) -> Option<Box<KDNode>> {
    if nodes_vec.is_empty() {
        return None;
    }

    let axis = match depth % 3 {
        0 => Coord::X,
        1 => Coord::Y,
        2 => Coord::Z,
        _ => Coord::X,
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

fn calculate_closest_node(
    node: &Option<Box<KDNode>>,
    goal_point: (i64, i64, i64),
    bests: KDNode,
) -> Option<Box<KDNode>> {

    match node {
        Some(n) => {
            
            let distance = n.distance(goal_point);
            //println!("Distance: ({:?}) -> ({:?}) {}", n.point, goal_point, distance);

            let mut best_copy = KDNode::new(bests.point, bests.axis);
            best_copy.left = bests.left;
            best_copy.right = bests.right;
            
            if distance < calc_distance(best_copy.point, goal_point) 
                && distance != 0. {
                best_copy.point = n.point;
                best_copy.axis = n.axis;
                best_copy.left = n.left.clone();
                best_copy.right = n.right.clone();
            }
            
            // if connection == [(162,817,812), (425,690,689)] {
            //     println!("OK SO NEW ?? {:?}", best_copy);
            // }
        

            let axis = n.axis;
            let diff = match axis {
                Coord::X => goal_point.0 - n.point.0,
                Coord::Y => goal_point.1 - n.point.1,
                Coord::Z => goal_point.2 - n.point.2,
            };

            let near = if diff < 0 { &n.left } else { &n.right };
            let far = if diff < 0 { &n.right } else { &n.left };

            
            if let Some(new_nearest) = calculate_closest_node(near, goal_point, best_copy.clone()) {
                
                best_copy.point = new_nearest.point;
                best_copy.axis = new_nearest.axis;
                best_copy.left = new_nearest.left.clone();
                best_copy.right = new_nearest.right.clone();
            }

            if (diff.abs() as f64) < calc_distance(bests.point, goal_point) {
                if let Some(new_nearest) = calculate_closest_node(far, goal_point, best_copy.clone()) {
                    
                    best_copy.point = new_nearest.point;
                    best_copy.axis = new_nearest.axis;
                    best_copy.left = new_nearest.left.clone();
                    best_copy.right = new_nearest.right.clone();
                }   
            }
            
            Some(Box::new(best_copy))
        }
        None => {
            Some(Box::new(bests))
        },
    }
}

fn calc_distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> f64 {
    ((((p1.0 - p2.0).pow(2))
        + ((p1.1 - p2.1).pow(2))
        + ((p1.2 - p2.2).pow(2))) as f64)
        .sqrt()
}
