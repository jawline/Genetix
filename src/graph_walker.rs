extern crate rand;

use std::collections::HashMap;
use std::vec::Vec;
use conn::Conn;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

/**
 * Generate a walk using dijkstras
 */
pub fn dijkstras(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
	let mut current = start;
	let mut result = Vec::new();

	result.push(start);

	while current != end {
		let ref paths = map[&current];
		let shortest = paths.iter().filter(|&x| result.iter().find(|&y| *y == x.dest).is_none()).min().unwrap();
		current = shortest.dest;
		result.push(current);
	}

	return result;
}

/**
 * Expensive approach at cutting the uneeded cycles from a walk
 */
 pub fn niave_cut_cycle(path : &mut Vec<i32>) {
    let mut instance_map = HashMap::<i32, i32>::new();
	
    for item in path.iter() {
        let current = instance_map.get(&item).cloned().or(Some(0)).unwrap();
        instance_map.insert(*item, current + 1);
    }
	
    loop {
        let start : usize;
        let end : usize;

    	if let Some((dest, _)) = instance_map.iter().find(|&(_, instance_map)| *instance_map > 1) {
            start = path.iter().position(|&x| x == *dest).unwrap();
            end = path.iter().rposition(|&x| x == *dest).unwrap();
        } else {
            break;
        }
        
        //Remove 1 count of each removed value
        for item in path.iter().skip(start).take(end - start) {
            if let Some(value) = instance_map.get_mut(&item) {
        	*value -= 1;
            }
        }
        
        let tail : Vec<i32> = path.iter().skip(end).cloned().collect();
        path.truncate(start);
        path.extend(tail);
    }
}

/**
 * Generate a random walk
 */
pub fn random_walk(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
	let mut result = Vec::new();
	let mut current = start;
   	let mut rng = rand::thread_rng();

   	result.push(start);

	while current != end {
		let ref paths = map[&current];
		let between = Range::new(0, paths.len());
		let rnum = between.ind_sample(&mut rng);

		//If we picked ourself go again
		if paths[rnum].dest != current {
			current = paths[rnum].dest;
			result.push(current);
		}
	}

	//Cut away any cycles from the walk
	niave_cut_cycle(result);
	return result;
}

/**
 * Generate a random walk without repeats
 */
pub fn random_walk_norepeat(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
	let mut result = Vec::new();
	let mut current = start;
   	let mut rng = rand::thread_rng();

   	result.push(start);

	while current != end {
		let ref paths = map[&current];
		let between = Range::new(0, paths.len());
		let rnum = between.ind_sample(&mut rng);
		current = paths[rnum].dest;

		//If we picked ourself go again
		if result.iter().find(|&x| *x == current).is_none() {
			result.push(current);
		}
	}

	return result;
}
