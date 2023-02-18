use std::io;
use std::fmt::Write;
use std::collections::VecDeque;


fn solve() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n: usize = n.trim().parse::<usize>().unwrap();

	let mut output = String::new();

	let mut visited: Vec<bool> = vec![false; n+1];
	visited[1] = true;
	let mut parent: Vec<u32> = vec![0; n+1];
	let mut v: Vec<Vec<usize>> = vec![Vec::new(); n+1];
	
	
	for i in 1..n {
		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		let mut line_iter = line.trim().split(" ").map(|x| x.parse::<usize>().unwrap());
		let first: usize = line_iter.next().unwrap();
		let second: usize = line_iter.next().unwrap();
		v[first].push(second);
		v[second].push(first);
	}

	let mut stack = VecDeque::new();
	
	stack.push_back(1);
	
	let mut dfs = || {
		while !stack.is_empty() {
			let x: usize = stack.pop_back().unwrap();
			for &adj in v[x].iter() {
				if !visited[adj] {
					visited[adj] = true;
					parent[adj] = x as u32;
					stack.push_back(adj);
				}
			}		
		}
	};
	dfs();
	let mut output = String::new();
	for i in 2..n+1 {
		writeln!(output, "{}", parent[i]);
	}
	print!("{}", output);
}


fn main() {
	solve();
}
