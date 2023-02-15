use std::io;
use std::collections::VecDeque;

fn solve() {
	let mut MNK = String::new();
	io::stdin().read_line(&mut MNK).unwrap();

	let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

	let MNK: Vec<usize> = MNK.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

	let M = MNK[0];
	let N = MNK[1];
	let K = MNK[2];
	
	let mut ground: Vec<Vec<bool>> = vec![vec![false; N];M];
	let mut cabbages: Vec<(i32, i32)> = Vec::new();
	let mut visited: Vec<Vec<bool>> = vec![vec![false; N];M];
	let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
	let mut cnt: u32 = 0;

	for _ in 0..K {
		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();

		let cabbage: Vec<i32> = line.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
		let cabbage: (i32, i32) = (cabbage[0], cabbage[1]);
		cabbages.push(cabbage);
		ground[cabbage.0 as usize][cabbage.1 as usize] = true;

	}
	let check = |x: i32, y: i32| {
		if x >= M as i32 || x < 0 || y >= N as i32 || y < 0 || ground[x as usize][y as usize] == false {
			false
		}
		else {
			true
		}
	};

	for &cabbage in cabbages.iter() {
		
		if visited[cabbage.0 as usize][cabbage.1 as usize] {
			continue;
		}
		cnt += 1;
		queue.push_back(cabbage);
		
		while !queue.is_empty() {
			let next = queue.pop_front().unwrap();
			let nextX = next.0;
			let nextY = next.1;

			if visited[nextX as usize][nextY as usize] {
				continue;
			} 

			visited[nextX as usize][nextY as usize] = true;

			for &direction in directions.iter() {
				let dx = direction.0;
				let dy = direction.1;
					
				if check(nextX + dx, nextY + dy) {
					queue.push_back((nextX + dx, nextY + dy));
				}
			}
		}
		
		
		queue.clear();
	}

	println!("{}", cnt);
}

fn main() {
	let mut tc_num = String::new();
	io::stdin().read_line(&mut tc_num).unwrap();
	let tc_num: usize = tc_num.trim().parse().unwrap();
	
	for i in 0..tc_num {
		solve();
	}
}
