mod conn;
mod graph_maker;
mod graph_printer;
mod graph_walker;

fn main() {
	let map = graph_maker::make_graph();
	graph_printer::output_connections(&map);
	let path = graph_walker::find_path(0, 30, &map);
	graph_printer::output_path(0, &path, &map);
}