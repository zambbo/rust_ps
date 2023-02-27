use std::io;

fn valid_next(map: &Vec<Vec<u32>>, point: (i64, i64), row_len: i64, col_len: i64) -> Vec<(i64, i64)> {
	let dx_dy: Vec<(i64, i64)> = vec![(1, 0), (0, -1), (0, 1), (-1, 0)];

	let next_vec: Vec<(i64, i64)> = dx_dy.iter().
						map(|x| (x.0 + point.0, x.1 + point.1)).
						filter(|x| x.0 >= 0 && x.1 >= 0 && x.0 < row_len && x.1 < col_len && map[x.0 as usize][x.1 as usize] < map[point.0 as usize][point.1 as usize])
						.collect::<Vec<(i64, i64)>>();
	next_vec
						
}

fn dfs(map: &Vec<Vec<u32>>, dp: &mut Vec<Vec<i64>>, point: (usize, usize), row_len:usize, col_len: usize) -> i64 {
	
	if point.0 == row_len -1 && point.1 == col_len -1 {
		dp[point.0][point.1] = 1;
	}
	

	let next_vec = valid_next(map, (point.0 as i64, point.1 as i64), row_len as i64, col_len as i64);

	if dp[point.0][point.1] == -1 {
		dp[point.0][point.1] = 0;
	} else{
		return dp[point.0][point.1];
	}	

	for &next_point in next_vec.iter() {
		dp[point.0][point.1] += dfs(map, dp, (next_point.0 as usize, next_point.1 as usize), row_len, col_len);
	}
	
	dp[point.0][point.1]
}

fn main() {
	let mut nm = String::new();
	io::stdin().read_line(&mut nm).unwrap();
	let mut nm_iter = nm.trim().split(" ").map(|x| x.parse::<usize>().unwrap());

	let m: usize = nm_iter.next().unwrap();
	let n: usize = nm_iter.next().unwrap();

	let mut map: Vec<Vec<u32>> = vec![vec![0;n];m];
	
	for i in 0..m {
		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		let line_iter = line.trim().split(" ").map(|x| x.parse::<u32>().unwrap());
		map[i] = line_iter.collect::<Vec<u32>>();
	}

	let mut dp: Vec<Vec<i64>> = vec![vec![-1;n];m];

	let ans: i64 = dfs(&map, &mut dp, (0, 0), m, n);

	println!("{}", ans as u64);
}
