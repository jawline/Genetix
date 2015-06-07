use std::collections::HashMap;
use std::vec::Vec;
use conn;

pub fn find_path(start : i32, end : i32, map : &HashMap<i32, Vec<conn::Conn>>) -> Vec<i32> {
	let mut current = start;
	let mut result = Vec::new();

	result.push(start);

	while current != end {
		
		let ref paths = map[&current];
		let mut shortest = &paths[0];

		for path in paths {
			let can_use = result.iter().find(|&x| *x == path.dest).is_none();
			if can_use && (path.cost < shortest.cost) {
				shortest = path;
			}
		}

		current = shortest.dest;
		result.push(current);
	}

	return result;
}