extern crate rand;

use std::collections::HashMap;
use std::vec::Vec;
use rand::Rng;
use conn;

pub fn make_graph() -> HashMap<i32, Vec<conn::Conn>> {
	let mut rng = rand::thread_rng();
	let mut map = HashMap::new();

	for x in 0..100 {
		let mut links = Vec::new();
		for y in 0..100 {
			if x != y {
				links.push(conn::Conn{dest: y, cost: (rng.gen::<i32>() % 150) + 151});
			}
		}
		map.insert(x, links);
	}

	return map;
}