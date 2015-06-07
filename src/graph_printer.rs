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