extern crate rand;

mod conn;
mod graph_maker;
mod graph_printer;

fn main() {
	let map = graph_maker::make_graph();
	graph_printer::output_connections(map);
}