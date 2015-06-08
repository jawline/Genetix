use std::collections::HashMap;
use std::vec::Vec;
use conn;

pub fn make_graph() -> HashMap<i32, Vec<conn::Conn>> {
	let mut map = HashMap::new();

	for x in 0..100 {
		let mut links = Vec::new();
		for y in 0..100 {
			links.push(conn::Conn{dest: y, cost: x == y ? 0 : y});
		}
		map.insert(x, links);
	}

	return map;
}
