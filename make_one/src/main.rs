use std::io;



fn main() {
	let mut Xstr = String::new();
	let min = |x, y| {
		if x > y {
			y
		}
		else {
			x
		}
	};

	io::stdin().read_line(&mut Xstr).unwrap();

	let X: usize = Xstr.trim().parse().unwrap();

	let mut dp: Vec<i32> = vec![0;X+1];
	
	dp[1] = 0;

	for i in 2..X+1 {
		if i % 3 == 0 {
			if i % 2 == 0 {
				dp[i] = min(dp[i/2] + 1, dp[i/3] + 1);
				dp[i] = min(dp[i], dp[i-1] + 1);
			}
			else {
				dp[i] = min(dp[i/3] + 1, dp[i-1] + 1);
			}
		}
		else if i % 2 == 0 {
			dp[i] = min(dp[i/2] + 1, dp[i-1] + 1);
		}
		else {
			dp[i] = dp[i-1] + 1;
		}
	}	

	println!("{}", dp[X]);
}
