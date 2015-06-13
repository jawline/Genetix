use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_printer;
use graph_walker;

fn walk_cost(from:i32, to:i32, path : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> i32 {
    let start = path.iter().position(|&x| x == from).unwrap();
    let end = path.iter().position(|&x| x == to).unwrap();
    
    let mut cost = 0;
    let mut current = start;
    
    while current != end {
        cost += graph_printer::cost(path[current], path[current+1], map);
        current += 1;
    }
    
    return cost;
}

fn combine_walk(left : &Vec<i32>, right: &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {

    for item in left {
        for other in left {
            if walk_cost(*item, *other, left, map) < walk_cost(*item, *other, right, map) {
                println!("Can reduce between {} and {}", item, other);
            }
        }
    }

    let mut result = Vec::new();
    result = left.iter().cloned().collect();
    return result;
}

/**
 * Generate a walk using dijkstras
 */
pub fn genetic(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
    let mut populations = Vec::new();
    
    //Generate initial populations
    for _ in 0..50 {
    	populations.push(graph_walker::random_walk(start, end, map));
    }

    const GENERATIONS : usize = 20;

    //Discard the worst 50% from the population and remake them, then resort, for GENERATIONS
    for _ in 0..GENERATIONS {
    	populations.sort_by(|x, y| graph_printer::total_cost(x, map).cmp(&graph_printer::total_cost(y, map)));
    	for i in 25..50 {
    		populations[i] = graph_walker::random_walk(start, end, map);
    	}
    }
    
    return populations.remove(0);
}
