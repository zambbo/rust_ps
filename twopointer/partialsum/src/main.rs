use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn solve() {
	let mut ns = String::new();
	io::stdin().read_line(&mut ns).unwrap();

	let mut ns_iter = ns.trim().split(" ").map(|x| x.parse::<usize>().expect("error when parse ns"));

	let N: usize = ns_iter.next().unwrap();
	let S: usize = ns_iter.next().unwrap();

	let mut array = String::new();
	io::stdin().read_line(&mut array).unwrap();

	let array: Vec<u32> = array.trim().split(" ").map(|x| x.parse::<u32>().expect("error when parse array")).collect();

	let mut start: usize = 0;
	let mut end: usize = 0;
	let mut cur_sum: u32 = 0;

	let mut min_heap = BinaryHeap::new();

	
	while end < N {
		if cur_sum >= S as u32 {
			min_heap.push(Reverse(end-start));
			cur_sum -= array[start];
			start += 1;
			continue;
		}
		cur_sum += array[end];
		end += 1;
	}

	while end == N && start < N {
		if cur_sum >= S as u32 {
			min_heap.push(Reverse(end-start));
			cur_sum -= array[start];
		}
		start += 1;
	}
	if min_heap.is_empty() {
		println!("0");
	}
	else {
		println!("{}", min_heap.pop().unwrap().0);
	}
}


fn main() {
	solve();
}
