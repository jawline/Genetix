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
	let mut dists = HashMap::new();
	let mut prev = HashMap::new();
	let mut untravelled = Vec::new();

	for (&item, _) in map {
		dists.insert(item, 9999999);
		untravelled.push(item);
	}

	dists.insert(start, 0);

	while untravelled.len() > 0 {
		untravelled.sort_by(|&x, &y| dists[&y].cmp(&dists[&x]));
		let u = untravelled.pop().unwrap();
		for neighbor in &map[&u] {
			let alt = dists[&u] + neighbor.cost;
			if alt < dists[&neighbor.dest] {
				dists.insert(neighbor.dest, alt);
				prev.insert(neighbor.dest, u);
			}
		}
	}

	let mut result = Vec::new();
	let mut cur = end;

	while cur != start {
		result.push(cur);
		cur = prev[&cur];
	}

	result.push(cur);
	result.reverse();

	return result;
}

/**
 * Expensive approach at cutting the uneeded cycles from a walk
 */
pub fn niave_cut_cycle(path : &mut Vec<i32>) {
	let mut instance_map = HashMap::<i32, i32>::new();

	for item in path.iter() {
		let current = instance_map.entry(*item).or_insert(0);
		*current += 1;
	}

	while let Some((&dest, _)) = instance_map.iter().find(|&(_, instance_map)| *instance_map > 1) {
		let start = path.iter().position(|&x| x == dest).unwrap();
		let end = path.iter().rposition(|&x| x == dest).unwrap();

		//Remove 1 count of each removed value
		for item in path.iter().skip(start).take(end - start) {
			if let Some(value) = instance_map.get_mut(&item) {
				*value -= 1;
			}
		}

		*path = path.iter().take(start).cloned().chain(path.iter().skip(end).cloned()).collect();
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
	niave_cut_cycle(&mut result);
	return result;
}

/**
 * Generate a random walk without repeats
 * NOTE: Only works on highly connected graphs - or if your lucky - will hang otherwise
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

		if result.iter().find(|&x| *x == current).is_none() {
			result.push(current);
		}
	}

	return result;
}
