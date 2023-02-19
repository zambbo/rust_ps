use std::io;


fn solve() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n: usize = n.trim().parse::<usize>().unwrap();

	let mut dp: Vec<Vec<u64>> = vec![vec![0; 10];101];
	for i in 0..10 {
		dp[1][i] = 1;
	}
	dp[1][0] = 0;
	for i in 2..101 {
		for j in 0..10 {
			if j == 0 {
				dp[i][j] += dp[i-1][j+1];	
				dp[i][j] %= 1_000_000_000;
			}
			else if j == 9 {
				dp[i][j] += dp[i-1][j-1];
				dp[i][j] %= 1_000_000_000;
			}
			else {
				dp[i][j] += dp[i-1][j-1] + dp[i-1][j+1];
				dp[i][j] %= 1_000_000_000;
			}
		}
	}
	//마지막 계산할때도 1_000_000_000으로 나누어 주어야 함 (sum 과정에서 overflow발생할 수 있기 때문에)
	println!("{}", (dp[n].iter().sum::<u64>())%1_000_000_000);

}

fn main() {
	solve();
}
