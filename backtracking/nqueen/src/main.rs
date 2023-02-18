use std::io;

static mut COUNTER: u32 = 0;

fn promising(ctx: &Vec<usize>) -> bool{
	if ctx.len() == 1 {
		return true;
	}
	for i in 0..ctx.len()-1 {
		if ctx[i] == *ctx.last().expect("error while last") {
			return false;
		}
		if ((ctx[i] as i32 - *ctx.last().expect("error while last") as i32) as i32).abs() == ((ctx.len() as i32-1-i as i32) as i32).abs() {
			return false;
		}
	}
	return true;
}


fn backtracking(ctx: &mut Vec<usize>, cur_row: usize, n: usize) {
	if cur_row == n {
		unsafe {
			COUNTER += 1;
		}
		return;
	}

	for col in 0..n {
		ctx.push(col);
		if promising(&ctx) {
			backtracking(ctx, cur_row + 1, n);
		}
		ctx.pop();
	}
}

fn main() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n: usize = n.trim().parse::<usize>().unwrap();
	let mut ctx = Vec::new();

	backtracking(&mut ctx, 0, n);
	unsafe {
		println!("{}", COUNTER);
	}
}
