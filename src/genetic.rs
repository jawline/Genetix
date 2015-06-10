use conn::Conn;
use std::collections::HashMap;
use std::vec::Vec;
use graph_printer;
use graph_walker;

/**
 * Generate a walk using dijkstras
 */
pub fn genetic(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
    let mut populations = [graph_walker::random_walk(start, end, map); 500];
    populations.sort_by(|&x &y| graph_printer::total_cost(x) < graph_printer::total_cost(y));
    return populations[0];
}
