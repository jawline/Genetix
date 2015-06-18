extern crate rand;

mod conn;
mod graph_maker;
mod graph_printer;
mod graph_walker;
mod genetic;

fn main() {
	let map = graph_maker::make_graph();
	
	let random_path = graph_walker::random_walk(0, map.iter().count() / 2, &map);
	graph_printer::output_path(&random_path, &map);

	let path = graph_walker::dijkstras(0, map.iter().count() / 2, &map);
	graph_printer::output_path(&path, &map);

	let genetic_path = genetic::genetic(0, map.iter().count() / 2, &map);
	graph_printer::output_path(&genetic_path, &map);
	
	println!("Optimal: {} Genetic: {} Random: {}", graph_printer::total_cost(&path, &map), graph_printer::total_cost(&genetic_path, &map), graph_printer::total_cost(&random_path, &map));
}
