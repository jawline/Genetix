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

fn cost(from : &i32, to : &i32, map : &HashMap<i32, Vec<conn::Conn>>) -> i32 {
	for conn in &map[from] {
		if conn.dest == *to {
			return conn.cost;
		}
	}
	return -1;
}

pub fn output_path(start : i32, path : &Vec<i32>, map : &HashMap<i32, Vec<conn::Conn>>) {
	let mut current = start;
	let mut total_cost = 0;
	for next in path {
		let cost = cost(&current, next, map);
		println!("From {} to {} (Cost {})", current, *next, cost);
		total_cost += cost;
		current = *next;
	}
	println!("Done (Total Cost: {})", total_cost);
}