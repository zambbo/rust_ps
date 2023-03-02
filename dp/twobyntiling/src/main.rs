use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse::<usize>().unwrap();

    let mut dp: Vec<usize> = vec![1; n+1];

    for i in 2..=n {
        dp[i] = (dp[i-1] + dp[i-2]) % 10_007;
    }

    println!("{}", dp[n] % 10_007);

}
