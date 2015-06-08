use std::collections::HashMap;
use std::vec::Vec;
use conn::Conn;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};
use graph_walker;
use graph_printer;

/**
 * Generate a walk using a genetic heuristic
 */
pub fn genetic(start : i32, end : i32, map : &HashMap<i32, Vec<Conn>>) -> Vec<i32> {
  const population_size : usize = 500;
  let initial [Vec<i32>, population_size] = [graph_walker::random_walk(start, end, map); population_size];
  let costs = initial.iter().map(|&x| graph_printer::total_cost(start, x, map));
}
