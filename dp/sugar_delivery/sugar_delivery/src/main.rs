use std::io;

fn min(n1: i32, n2: i32) -> i32 {
	if n1 > n2 {
		n2
	}
	else {
		n1
	}
}

fn main() {
	let mut Nstr = String::new();
	io::stdin().read_line(&mut Nstr).unwrap();
	let N: usize = Nstr.trim().parse().unwrap();
	
	let mut dp: Vec<i32> = vec![0;5001];
	dp[3] = 1;
	dp[5] = 1;
	for i in 6..N+1 {
		if dp[i-3] != 0 {
			dp[i] = dp[i-3] + 1;
		}	
		if dp[i-5] != 0 {
			if dp[i] != 0 {
				dp[i] = min(dp[i], dp[i-5] + 1);
			}
			else {
				dp[i] = dp[i-5] + 1;
			}
		}
	}	
	if dp[N] == 0 {
		dp[N] = -1;
	}
	println!("{}", dp[N]);
	
}
