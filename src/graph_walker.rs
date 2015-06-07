use std::collections::HashMap;
use std::vec::Vec;
use conn::Conn;

fn first_usable<'a>(current : &'a Vec<Conn>, result : &Vec<i32>) -> Option<&'a Conn> {
	return current.iter().find(|&x| result.iter().find(|&y| *y == x.dest).is_none());
}

pub fn find_path(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
	let mut current = start;
	let mut result = Vec::new();

	result.push(start);

	while current != end {
		
		let ref paths = map[&current];
		let mut shortest = first_usable(paths, &result).unwrap();

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