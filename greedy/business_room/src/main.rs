use std::io;


fn main() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n: usize = n.trim().parse().unwrap();
	let mut classes: Vec<(i32, i32)> = Vec::new();

	let get_class = || {
		let mut class_time = String::new();
		io::stdin().read_line(&mut class_time).unwrap();
		class_time.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
	};

	for i in 0..n {
		let class: Vec<i32> = get_class();
		classes.push((class[0], class[1]));
	}
	classes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
	classes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
	
	let mut cnt = 0;
	let mut idx = 0;
	loop {
		if idx >= n {
			break;
		}	
		let last_class_end_time = classes[idx].1;
		cnt+=1;
		idx+=1;
		while idx < n && classes[idx].0 < last_class_end_time {
			idx += 1;
		}
	}
	
	println!("{}", cnt);
}
