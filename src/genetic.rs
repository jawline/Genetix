use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_printer;
use graph_walker;

fn walk_cost(start_op:Option<usize>, end_op:Option<usize>, path : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<i32> {
    
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

fn longest_reduction(left : &Vec<i32>, right : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<(i32, (usize, usize), (usize, usize))> {
    let mut longestReduction = None;
    
    for x in 0..left.len() {
        for y in x + 1..left.len() {
            let (right_start, right_end) = (right.iter().position(|&i| i == left[x]), right.iter().position(|&i| i == left[y]));
            let (leftCost, rightCost) = (walk_cost(Some(x), Some(y), left, map), walk_cost(right_start, right_end, right, map));
            if leftCost != None && rightCost != None {
	        let reduction = -(rightCost.unwrap() - leftCost.unwrap());
	        
	        let currentReduction = match longestReduction {
	        	None => 0,
	        	Some((reduction, _, _)) => reduction
	        };
	        
	        if reduction > currentReduction {
	            let leftPositions = (x,y);
	            let rightPositions = (right_start.unwrap(), right_end.unwrap());
	            longestReduction = Some((reduction, leftPositions, rightPositions));
	        }
	    }
        }
    }
    
    return longestReduction;
}

fn combine_walk(left : &Vec<i32>, right: &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<Vec<i32>> {
    return match longest_reduction(left, right, map) {
    	None => None,
    	Some((reduced_by, (left_start, left_end), (right_start, right_end))) => 
    		Some(left.iter().cloned().take(left_start).chain(
    			right.iter().cloned().skip(right_start).take(right_end - right_start)
    		).chain(
    			left.iter().cloned().skip(left_end)
    		).collect())
    }
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

    const COMBINE_AMOUNT : usize = 10;
    const GENERATIONS : usize = 40;

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
