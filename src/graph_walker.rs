use std::collections::HashMap;
use std::vec::Vec;
use conn;

pub fn find_path(start : i32, end : i32, map : &HashMap<i32, Vec<conn::Conn>>) -> Vec<i32> {
	let mut current = start;
	let mut result = Vec::new();

	while current != end {
		let ref paths = map[&current];
		let mut shortest = *paths[0];
		for path in paths {
			if path.cost < shortest.cost {
				shortest = *path;
			}
		}
		println!("{}", shortest.dest);
	}

	return result;
}