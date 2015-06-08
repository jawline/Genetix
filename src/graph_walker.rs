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
	let mut count = HashMap::new();
	
	//Some helper functions to recount the number of elements in old and say if we are done
	fn recount() {
		for item in path {
			count[item] += 1;
		}
	}
	
	fn cut(from : usize, to : usize) {
		for x in from..to {
			path.remove(from);
		}
	}
	
	recount();
	
	let mut it = count.iter().find(|&x| *x > 1);
	
	while !it.is_none() {
		let (item, _) = it.unwrap();
		let start = old.position(|&x| *x == item);
		let end = old.rposition(|&x| *x == item);
		cut(&old, start, end);

		//Recalculate the count map and look for the next iter (We may have inadvertantly removed doubles already)
		recount();
		it = count.iter().find(|&x| *x > 1);
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
