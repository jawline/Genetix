use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_printer;
use graph_walker;

/**
 * Function that returns the cost of the walk between two indices
 */
fn subwalk_cost(start_op:Option<usize>, end_op:Option<usize>, path : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<i32> {
    
    if start_op == None || end_op == None {
    	return None;
    }
    
    let (start, end) = (start_op.unwrap(), end_op.unwrap());
    
    if start < end {
	let mut cost = 0;
	let mut current = start;
	while current != end {
		cost += graph_printer::cost(path[current], path[current+1], map);
		current += 1;
	}
	return Some(cost);
   }

    return None;
}

/**
 * Function that finds the longest viable reduction in a walk
 */
fn longest_reduction(left : &Vec<i32>, right : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<(i32, (usize, usize), (usize, usize))> {
    let mut best_reduction = None;
    
    for x in 0..left.len() {
        for y in x + 1..left.len() {
            let (right_start, right_end) = (right.iter().position(|&i| i == left[x]), right.iter().position(|&i| i == left[y]));
            let (left_walk_cost, right_walk_cost) = (subwalk_cost(Some(x), Some(y), left, map), subwalk_cost(right_start, right_end, right, map));
            if left_walk_cost != None && right_walk_cost != None {
	        let amount_reduced = -(right_walk_cost.unwrap() - left_walk_cost.unwrap());
	        
	        let best_reduction_amount = match best_reduction {
	        	None => 0,
	        	Some((reduction, _, _)) => reduction
	        };
	        
	        if amount_reduced > best_reduction_amount {
	            let left_positions = (x,y);
	            let right_positions = (right_start.unwrap(), right_end.unwrap());
	            best_reduction = Some((amount_reduced, left_positions, right_positions));
	        }
	    }
        }
    }
    
    return best_reduction;
}

/**
 * Attempt to combine the best features of two walks
 */
fn combine_walk(left : &Vec<i32>, right: &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<Vec<i32>> {
    return match longest_reduction(left, right, map) {
    	None => None,
    	Some((_, (left_start, left_end), (right_start, right_end))) => 
    		Some(left.iter().cloned().take(left_start).chain(
    			right.iter().cloned().skip(right_start).take(right_end - right_start)
    		).chain(
    			left.iter().cloned().skip(left_end)
    		).collect())
    };
}

/**
 * Generate a walk using a weird genetic algorithm
 */
pub fn genetic(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
    let mut populations = Vec::new();
    
    //Generate initial populations
    for _ in 0..50 {
    	populations.push(graph_walker::random_walk(start, end, map));
    }

    const COMBINE_AMOUNT : usize = 10;
    const GENERATIONS : usize = 10;

    //Discard the worst 50% from the population and remake them, then resort, for GENERATIONS
    for gen in 0..GENERATIONS {
    	populations.sort_by(|x, y| graph_printer::total_cost(y, map).cmp(&graph_printer::total_cost(x, map)));

    	let mut combined = populations.pop().unwrap();

	println!("Before Reduce: {}", graph_printer::total_cost(&combined, map));

    	for item in populations.iter() {
    		if let Some(improved) = combine_walk(&combined, item, map) {
    			combined = improved;
    		}
    	}
    	
    	println!("After Reduce: {}", graph_printer::total_cost(&combined, map));

    	for item in populations.iter_mut() {
    		*item = graph_walker::random_walk(start, end, map);
    	}

    	populations.push(combined);
    	println!("Gen {} done", gen);
    }
    
    return populations.pop().unwrap();
}
