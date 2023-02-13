use std::io;


fn main() {
	let mut Nstr = String::new();
	io::stdin().read_line(&mut Nstr).unwrap();
	let N: usize = Nstr.trim().parse().unwrap();

	let mut dp: Vec<(u64, u64)> = vec![(0,0); 91]; // (0으로 끝나는 이친수, 1으로 끝나는 이친수)
	
	dp[1] = (0, 1);
	dp[2] = (1, 0);
	dp[3] = (1, 1);

	for i in 4..N+1 {
		dp[i] = (dp[i-1].0 + dp[i-1].1, dp[i-1].0);	
	}
	
	println!("{}", dp[N].0 + dp[N].1);
}
