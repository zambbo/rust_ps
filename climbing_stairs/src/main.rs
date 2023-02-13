use std::io;

fn main() {
	let mut Nstr = String::new();

	io::stdin().read_line(&mut Nstr).unwrap();
	let N: usize = Nstr.trim().parse().unwrap();

	let max = |x, y| {
		if x > y {
			x
		}
		else {
			y
		}
	};	

	let mut dp = vec![(0, 0); N+1];
	let mut scores = vec![0; N+1];
	for i in 1..N+1 {
		let mut score_str = String::new();
		io::stdin().read_line(&mut score_str).unwrap();
		let score: i32 = score_str.trim().parse().unwrap();

		scores[i] = score;
	}

	dp[0] = (0, 0);
	dp[1] = (scores[1], scores[1]); // (1번째 전에서 뛴거, + 2번째 전에서 뛴거)
	for i in 2..N+1 {
		dp[i].0 = dp[i-1].1 + scores[i];
		dp[i].1 = max(dp[i-2].0 + scores[i], dp[i-2].1 + scores[i]);
	}

	println!("{:?}", max(dp[N].0, dp[N].1));
}
