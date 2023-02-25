use std::io;

fn solve() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n: usize = n.trim().parse::<usize>().unwrap();

	let mut dp: Vec<i32> = vec![i32::MIN; n];
	
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	let sequence: Vec<i32> = line.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

	let mut max_sum: i32 = 0;
	dp[0] = sequence[0];
	for i in 1..n {
		if dp[i-1] + sequence[i] > sequence[i] {
			dp[i] = dp[i-1] + sequence[i];
		}
		else {
			dp[i] = sequence[i];
		} 
	}

	max_sum = *dp.iter().max().unwrap();

	println!("{}", max_sum);

}

fn main() {
	solve();
}
