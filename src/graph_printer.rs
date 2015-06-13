use std::collections::HashMap;
use std::vec::Vec;
use conn;

pub fn output_connections(map : &HashMap<i32, Vec<conn::Conn>>) {
	for map_entry in map {
		let (from, links) = map_entry;
		for link in links {
			println!("Map {} to {} cost {}", from, link.dest, link.cost);
		}
	}
}

pub fn cost(from : i32, to : i32, map : &HashMap<i32, Vec<conn::Conn>>) -> i32 {
	for conn in &map[&from] {
		if conn.dest == to {
			return conn.cost;
		}
	}
	return -1;
}

pub fn total_cost(path : &Vec<i32>, map : &HashMap<i32, Vec<conn::Conn>>) -> i32 {
	let mut total_cost = 0;
	let mut last = path[0];

	for &next in path.iter().skip(1) {
		total_cost += cost(last, next, map);
		last = next;
	}

	return total_cost;
}

pub fn output_path(path : &Vec<i32>, map : &HashMap<i32, Vec<conn::Conn>>) {
	let mut last = path[0];

	for &next in path.iter().skip(1) {
		let cost = cost(last, next, map);
		println!("From {} to {} (Cost {})", last, next, cost);
		last = next;
	}

	println!("Done (Total Cost: {})", total_cost(path, map));
}
