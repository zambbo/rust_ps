use std::io;


fn main() {
	let mut Nstr = String::new();
	io::stdin().read_line(&mut Nstr).unwrap();

	let N: i32 = Nstr.trim().parse().unwrap();
	
	
	for i in 0..N {
		let mut Nstr = String::new();
		io::stdin().read_line(&mut Nstr).unwrap();
		
		let n: usize = Nstr.trim().parse().unwrap();

		let veclen_closer = |x| {
			if x > 3 {
				x
			}
			else {
				3
			}
		};
		let veclen: usize = veclen_closer(n);
		let mut dp: Vec<i32> = vec![0; veclen+1];

		
		dp[1] = 1;
		dp[2] = 2;
		dp[3] = 4;

		for i in 4..n+1 {
			dp[i] = dp[i-1] + dp[i-2] + dp[i-3];
		}
		println!("{}", dp[n]);
	}
}
