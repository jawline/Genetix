extern crate rand;

mod graph_maker;
mod conn;

fn main() {
	let map = graph_maker::make_graph();
	for map_entry in map {
		let (_, links) = map_entry;
		for link in links {
			println!("{} {}", link.dest, link.cost);
		}
	}
}