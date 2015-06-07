extern crate rand;

use std::collections::HashMap;
use rand::Rng;

mod other;

struct Conn {
	dest: i32,
	cost: i32
}

fn main() {
	println!("Hello World!\n");
	other::poo();

	let mut rng = rand::thread_rng();

	let mut map = HashMap::new();

	for x in 0..100 {
		for y in 0..100 {
			map.insert(x, Conn{dest: y, cost: rng.gen::<i32>() % 150});
		}
	}

	for link in map {
		let (_, x) = link;
		println!("{} {}", x.dest, x.cost);
	}
}