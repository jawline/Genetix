extern crate rand;

use std::collections::HashMap;
use std::vec::Vec;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};
use conn;

pub fn make_graph() -> HashMap<i32, Vec<conn::Conn>> {
	let mut map = HashMap::new();
	let mut rng = rand::thread_rng();
	
	const max_link_cost : i32 = 8000;

	for x in 0..1000 {
		let mut links = Vec::new();
		for y in 0..100 {
			if y == x {
				links.push(conn::Conn{dest: y, cost: 0});
			} else {
				links.push(conn::Conn{
					dest: y,
					cost: Range::new(0, max_link_cost).ind_sample(&mut rng)
				});
			}
		}
		map.insert(x, links);
	}

	return map;
}
