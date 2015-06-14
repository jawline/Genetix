use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_printer;
use graph_walker;

fn walk_cost(from:i32, to:i32, path : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<i32> {
    let start_op = path.iter().position(|&x| x == from);
    let end_op = path.iter().position(|&x| x == to);

    let mut result : Option<i32> = None;

    if let Some(start) = start_op {
    	if let Some(end) = end_op {
    		if start < end {
	    	    let mut cost = 0;
			    let mut current = start;
			    
			    while current != end {
			        cost += graph_printer::cost(path[current], path[current+1], map);
			        current += 1;
			    }

			    result = Some(cost);
		    }
    	}
    }
    
    return result;
}

fn combine_walk(left : &Vec<i32>, right: &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {

    for x in 0..left.len() {
        for y in x + 1..left.len() {
            if walk_cost(left[x], left[y], left, map) < walk_cost(left[x], left[y], right, map) {
                println!("Can reduce between {} and {}", left[x], left[y]);
            }
        }
    }

    let result = left.iter().cloned().collect();
    return result;
}

/**
 * Generate a walk using dijkstras
 */
pub fn genetic(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
    let mut populations = Vec::new();
    
    //Generate initial populations
    for _ in 0..25 {
    	populations.push(graph_walker::random_walk(start, end, map));
    }

    const GENERATIONS : usize = 20;

    //Discard the worst 50% from the population and remake them, then resort, for GENERATIONS
    for _ in 0..GENERATIONS {
    	populations.sort_by(|x, y| graph_printer::total_cost(y, map).cmp(&graph_printer::total_cost(x, map)));

    	let mut combined = populations.pop().unwrap();

    	for item in populations.iter().take(populations.len() - 1) {
    		combine_walk(&combined, item, map);
    	}

    	for item in populations.iter_mut().rev().skip(1) {
    		*item = graph_walker::random_walk(start, end, map);
    	}

    	populations.push(combined);
    }
    
    return populations.pop().unwrap();
}
