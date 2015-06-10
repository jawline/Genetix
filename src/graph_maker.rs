use std::collections::HashMap;
use std::vec::Vec;
use conn;

pub fn make_graph() -> HashMap<i32, Vec<conn::Conn>> {
	let mut map = HashMap::new();

	for x in 0..1000 {
		let mut links = Vec::new();
		for y in 0..100 {
			if y == x {
				links.push(conn::Conn{dest: y, cost: 0});
			} else {
				if x == 0 && y == 30 {
					links.push(conn::Conn{dest: y, cost: 999999});
				} else if x == 0 || x % 2 != 0 {
					links.push(conn::Conn{dest: y, cost: y});
				} else {
					links.push(conn::Conn{dest: y, cost: 999999});
				}
			}
		}
		map.insert(x, links);
	}

	return map;
}
