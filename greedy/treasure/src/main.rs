use std::io;

fn main() {
	let mut Nstr = String::new();
	io::stdin().read_line(&mut Nstr).unwrap();
	let N: usize = Nstr.trim().parse().unwrap();
	let parse_to_vec = |x: String| {
		x.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
	};
	let mut A = String::new();
	io::stdin().read_line(&mut A).unwrap();
	let mut A: Vec<i32> = parse_to_vec(A);
	let mut B = String::new();
	io::stdin().read_line(&mut B).unwrap();
	let mut B: Vec<i32> = parse_to_vec(B);
	A.sort();
	B.sort();	
	let mut ans: i32 = 0;
	for (a, b) in A.iter().zip(B.iter().rev()) {
		ans += a*b;
	}
	println!("{}", ans);
}


