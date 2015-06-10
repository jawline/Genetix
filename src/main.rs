extern crate rand;

mod conn;
mod graph_maker;
mod graph_printer;
mod graph_walker;
mod genetic;

fn main() {
	let map = graph_maker::make_graph();
	
	graph_printer::output_connections(&map);

	let path = graph_walker::dijkstras(0, 30, &map);
	graph_printer::output_path(&path, &map);
	
	let random_path = graph_walker::random_walk(0, 30, &map);
	graph_printer::output_path(&random_path, &map);
	
	let random_path_norepeat = graph_walker::random_walk_norepeat(0, 30, &map);
	graph_printer::output_path(&random_path_norepeat, &map);

	let genetic_path = genetic::genetic(0, 30, &map);
	graph_printer::output_path(&genetic_path, &map);
}