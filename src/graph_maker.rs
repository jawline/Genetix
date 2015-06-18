extern crate rand;

use std::collections::HashMap;
use std::vec::Vec;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};
use conn;

pub fn make_graph() -> HashMap<i32, Vec<conn::Conn>> {
	let mut map = HashMap::new();
	let mut rng = rand::thread_rng();
	
	const MAX_LINK_COST : i32 = 8000;
	const MAX_ITEMS : i32 = 1000;
	const MAX_LINKS_PER_ITEM : i32 = 400;
	
	let number_of_items = Range::new(1, MAX_ITEMS).ind_sample(&mut rng);

	for x in 0..number_of_items {
		let number_of_links = Range::new(1, MAX_LINKS_PER_ITEM).ind_sample(&mut rng);
		let mut links = Vec::new();
		for y in 0..number_of_links {
			if y == x {
				links.push(conn::Conn{dest: y, cost: 0});
			} else {
				links.push(conn::Conn{
					dest: y,
					cost: Range::new(0, MAX_LINK_COST).ind_sample(&mut rng)
				});
			}
		}
		map.insert(x, links);
	}

	return map;
}
