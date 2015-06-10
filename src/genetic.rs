use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_walker;

/**
 * Generate a walk using dijkstras
 */
pub fn genetic(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
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
