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
  let mut populations [Vec<i32>, population_size] = [graph_walker::random_walk(start, end, map); population_size];
  populations.sort_by(|&x, &y| cost(start, x, map) < cost(start, y, map));
}
