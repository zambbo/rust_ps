use std::io;
use std::collections::VecDeque;

fn main() {
	let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
	let mut npm = String::new();
	io::stdin().read_line(&mut npm).unwrap();
	let npm: Vec<usize> = npm.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
	let N: usize = npm[0];
	let M: usize = npm[1];
	
	let mut maze: Vec<Vec<i32>>= vec![vec![0; M]; N];
	let mut visited: Vec<Vec<bool>> = vec![vec![false; M]; N];

	let mut queue: VecDeque<((i32, i32), u32)> = VecDeque::new();
	
	
	for i in 0..N {
		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		let digits: Vec<u32> = line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
		for (j, &digit) in digits.iter().enumerate() {
			maze[i][j] = digit as i32;
		}
	}	
	
	let check = |x: i32, y: i32| {
		if x >= N as i32 || x < 0 || y >= M as i32 || y < 0 || maze[x as usize][y as usize] == 0{
			false
		} 
		else {
			true
		}
	};

	queue.push_back(((0, 0), 1));

	while !queue.is_empty() {
		let next = queue.pop_front().unwrap();
		let nextX = next.0.0;
		let nextY = next.0.1;
		let cnt = next.1;
		if visited[nextX as usize][nextY as usize] {
			continue;
		}
		visited[nextX as usize][nextY as usize] = true;

		if nextX == N as i32 -1 && nextY == M as i32 -1{
			println!("{}", cnt);
			break;
		}

		for &direction in directions.iter() {
			let dx = direction.0;
			let dy = direction.1;
			let nextXX = nextX + dx;
			let nextYY = nextY + dy;
			if check(nextXX, nextYY) {
				queue.push_back(((nextXX, nextYY), cnt + 1));
			}	
		}

		
	}	
}


