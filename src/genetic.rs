use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_printer;
use graph_walker;

fn walk_cost(from:i32, to:i32, path : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<i32> {
    let fromPos = path.iter().position(|&x| x == from);
    let toPos = path.iter().position(|&x| x == to);

    if fromPos != None && toPos != None {
	let (start, end) = (fromPos.unwrap(), toPos.unwrap());
	if start < end {
	        let mut cost = 0;
		let mut current = start;
		while current != end {
			cost += graph_printer::cost(path[current], path[current+1], map);
			current += 1;
		}
		return Some(cost);
	}
    }

    return None;
}

fn longest_reduction(left : &Vec<i32>, right : &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Option<(i32, (usize, usize), (usize, usize))> {
    let mut longestReduction = None;
    
    for x in 0..left.len() {
        for y in x + 1..left.len() {
            let (leftCost, rightCost) = (walk_cost(left[x], left[y], left, map), walk_cost(left[x], left[y], right, map));
            if leftCost != None && rightCost != None {
	        let reduction = -(rightCost.unwrap() - leftCost.unwrap());
	        
	        let currentReduction = match longestReduction {
	        	None => 0,
	        	Some((reduction, _, _)) => reduction
	        };
	        
	        if reduction > currentReduction {
	            println!("Marked Reduction {} {} {}", leftCost.unwrap(), rightCost.unwrap(), reduction);
	            let leftPositions = (left.iter().position(|&i| i == left[x]).unwrap(), left.iter().position(|&i| i == left[y]).unwrap());
	            let rightPositions = (right.iter().position(|&i| i == left[x]).unwrap(), right.iter().position(|&i| i == left[y]).unwrap());
	            longestReduction = Some((reduction, leftPositions, rightPositions));
	        }
	    }
        }
    }
    
    return longestReduction;
}

fn combine_walk(left : &Vec<i32>, right: &Vec<i32>, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
    return match longest_reduction(left, right, map) {
    	None => left.iter().cloned().collect(),
    	Some((reduced_by, (left_start, left_end), (right_start, right_end))) => {
    		println!("Reduce {} Indices: ({},{}) ({},{})", reduced_by, left_start, left_end, right_start, right_end);
    		println!("Left {}", walk_cost(left[left_start], left[left_end], left, map).unwrap());
    		println!("Right {}", walk_cost(left[left_start], left[left_end], right, map).unwrap());
    		left.iter().cloned().take(left_start).chain(
    			right.iter().cloned().skip(right_start).take(right_end - right_start)
    		).chain(
    			left.iter().cloned().skip(left_end)
    		).collect()
    	}
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
    const GENERATIONS : usize = 20;

    //Discard the worst 50% from the population and remake them, then resort, for GENERATIONS
    for gen in 0..GENERATIONS {
    	populations.sort_by(|x, y| graph_printer::total_cost(y, map).cmp(&graph_printer::total_cost(x, map)));

    	let mut combined = populations.pop().unwrap();

	println!("Before Reduce: {}", graph_printer::total_cost(&combined, map));

    	for item in populations.iter().skip(COMBINE_AMOUNT - 1).take(COMBINE_AMOUNT) {
    		combined = combine_walk(&combined, item, map);
    	}
    	
    	println!("After Reduce: {}", graph_printer::total_cost(&combined, map));

    	for item in populations.iter_mut().rev().skip(1) {
    		*item = graph_walker::random_walk(start, end, map);
    	}

    	populations.push(combined);
    	println!("Gen {} done", gen);
    }
    
    return populations.pop().unwrap();
}
