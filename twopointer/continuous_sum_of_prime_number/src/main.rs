use std::io;

fn eratos(n: usize) -> Vec<usize> {
	let mut primes: Vec<bool> = vec![true; n+1];

	let m: usize = n ^ 1/2;

	for i in 2..m+1 {
		if primes[i] {
			for j in (i*2..n+1).step_by(i) {
				primes[j] = false;
			}
		}
	}
	primes[0] = false;
	primes[1] = false;
	let primes: Vec<usize> = primes.iter().
				enumerate().
				map(|(i, &x)| {
					if x == true {
						i
					}
					else {
						0
					}		
				}).
				filter(|&x| x != 0).
				collect();
	primes
}

fn solve(primes: &Vec<usize>, N: usize) {
	let mut start: usize = 0;
	let mut end: usize = 0;
	let mut cnt: u32 = 0;
	let mut cur_sum: usize = 0;
	let prime_len: usize = primes.len();

	while end < prime_len {
		if cur_sum == N {
			cnt += 1;
		}
		if cur_sum >= N {
			cur_sum -= primes[start];
			start += 1;
		}
		else {
			cur_sum += primes[end];
			end += 1;	
		}
	}

	while start < prime_len {
		if cur_sum == N {
			cnt += 1;
			break;
		}
		cur_sum -= primes[start];
		start += 1;
	}
	println!("{}", cnt);
}


fn main() {

	let mut N = String::new();
	io::stdin().read_line(&mut N).unwrap();
	let N:usize = N.trim().parse::<usize>().expect("error while parse N");
	let prime_numbers: Vec<usize> = eratos(N);
	solve(&prime_numbers, N);
}
