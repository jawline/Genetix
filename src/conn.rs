use std::cmp;
use std::cmp::Ordering;

pub struct Conn {
	pub dest: i32,
	pub cost: i32
}

impl cmp::PartialOrd for Conn {
	fn partial_cmp(&self, other: &Conn) -> Option<Ordering> {
		return Some(self.dest.cmp(&other.dest));
	}
}
impl cmp::PartialEq for Conn {
	fn eq(&self, other: &Conn) -> bool {
		return self.dest == other.dest;
	}
}

impl cmp::Eq for Conn {
}

impl cmp::Ord for Conn {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.dest.cmp(&other.dest);
	}
}