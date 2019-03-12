// Generate a random n by n tree (acyclic graph)
// 2019 3 10
// Owen Lange

use rand::prelude::*;
use std::env;

fn rcn_to_i(r: usize, c: usize, n: usize) -> usize {
	(r * n) + c
}

struct Node {
	group: usize,
	// these bools refer to whether the connecting segment in that direction is use in the tree.
	north: bool,
	south: bool,
	east: bool,
	west: bool,
}

struct Edge {
	// these whole numbers are indices in the Node vec.
	one_end: usize,
	other_end: usize,
	vertical: bool,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let n: usize = match args.get(1) {
		Some(arg) => {
			match arg.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Command line argument for size could not be parsed. Using 0");
					0
				}
			}
		},
		None => {
			println!("Command line argument for size of maze was not found. Using 0.");
			0
		}
	};
	// println!("About ready to start the actual maze generation. Maze size taken in as {}.", n);
	let mut nodes = Vec::with_capacity(n*n);
	for row in 0..n {
		for col in 0..n {
			nodes.push(Node{
				group: rcn_to_i(row, col, n),
				north: false,
				south: false,
				east: false,
				west: false,
			});
		}
	}
	let mut edges = Vec::with_capacity(2*n*(n-1));
	// vertical edges
	for row in 0..(n-1) {
		for col in 0..n {
			edges.push(Edge{
				one_end: rcn_to_i(row, col, n),
				other_end: rcn_to_i(row+1, col, n),
				vertical: true,
			});
		}
	}
	// horizontal edges
	for row in 0..n {
		for col in 0..(n-1) {
			edges.push(Edge{
				one_end: rcn_to_i(row, col, n),
				other_end: rcn_to_i(row, col+1, n),
				vertical: false,
			});
		}
	}
	let mut rng = thread_rng();
	edges.shuffle(&mut rng);
	for edge in edges {
		if nodes[edge.one_end].group != nodes[edge.other_end].group {
			if edge.vertical {
				nodes[edge.one_end].south = true;
				nodes[edge.other_end].north = true;
			} else {
				nodes[edge.one_end].east = true;
				nodes[edge.other_end].west = true;
			}
			let new_group = nodes[edge.one_end].group;
			let old_group = nodes[edge.other_end].group;
			for mut node in &mut nodes {
				if node.group == old_group {
					node.group = new_group;
				}
			}
		}
	}
	// let mut diagram = Vec::with_capacity(3*n);
	// for row in 0..n {
	// 	diagram.push(String::from(""));
	// 	diagram.push(String::from(""));
	// 	diagram.push(String::from(""));
	// 	for col in 0..n {
	// 		let my_node = &nodes[rcn_to_i(row, col, n)];
	// 		if my_node.north {
	// 			diagram[row*3].push_str("  |  ");
	// 		} else {
	// 			diagram[row*3].push_str("     ");
	// 		}
	// 		if my_node.west {
	// 			diagram[row*3+1].push_str("--");
	// 		} else {
	// 			diagram[row*3+1].push_str("  ");
	// 		}
	// 		diagram[row*3+1].push_str("+");
	// 		if my_node.east {
	// 			diagram[row*3+1].push_str("--");
	// 		} else {
	// 			diagram[row*3+1].push_str("  ");
	// 		}
	// 		if my_node.south {
	// 			diagram[row*3+2].push_str("  |  ");
	// 		} else {
	// 			diagram[row*3+2].push_str("     ");
	// 		}
	// 	}
	// }
	let mut diagram = Vec::with_capacity(n);
	for row in 0..n {
		diagram.push(String::from(""));
		for col in 0..n {
			let my_node = &nodes[rcn_to_i(row, col, n)];
			if my_node.north {
				if my_node.south {
					if my_node.east {
						if my_node.west {
							diagram[row].push_str("╋");
						} else {
							diagram[row].push_str("┣");
						}
					} else {
						if my_node.west {
							diagram[row].push_str("┫");
						} else {
							diagram[row].push_str("┃");
						}
					}
				} else {
					if my_node.east {
						if my_node.west {
							diagram[row].push_str("┻");
						} else {
							diagram[row].push_str("┗");
						}
					} else {
						if my_node.west {
							diagram[row].push_str("┛");
						} else {
							diagram[row].push_str("╹");
						}
					}
				}
			} else {
				if my_node.south {
					if my_node.east {
						if my_node.west {
							diagram[row].push_str("┳");
						} else {
							diagram[row].push_str("┏");
						}
					} else {
						if my_node.west {
							diagram[row].push_str("┓");
						} else {
							diagram[row].push_str("╻");
						}
					}
				} else {
					if my_node.east {
						if my_node.west {
							diagram[row].push_str("━");
						} else {
							diagram[row].push_str("╺");
						}
					} else {
						if my_node.west {
							diagram[row].push_str("╸");
						} else {
							diagram[row].push_str("░");
						}
					}
				}
			}
		}
	}
	for gram in diagram {
		println!("{}", gram);
	}
}
