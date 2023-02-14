use std::io;

struct Queue {
	array: Box<[i32]>,
	front: i32,
	back: i32,
	max_length: i32,
}

impl Queue {
	fn new() -> Queue {
		Queue {
			array: Box::new([0; 10001]),
			front: 0,
			back: 0,
			max_length: 10001,
		}
	}

	fn empty(&self) -> i32{
		if self.front == self.back {
			1
		}
		else {
			0
		}
	}
	
	fn full(&self) -> i32 {
		if (self.back + 1) % self.max_length == self.front {
			1
		}
		else {
			0
		}
	}

	fn push(&mut self, data: i32) {
		if Self::full(&self) == 1{
			return;
		}
		
		self.back = (self.back + 1) % self.max_length;
		self.array[self.back as usize] = data;
	}

	fn pop(&mut self) -> i32 {
		if Self::empty(&self) == 1{
			return -1;
		}

		self.front = (self.front + 1) % self.max_length;
		self.array[self.front as usize]
	}

	fn _front(&self) -> i32 {
		if Self::empty(&self) == 1{
			return -1;
		}
		
		self.array[((self.front + 1) % self.max_length) as usize]	
	}

	fn _back(&self) -> i32 {
		if Self::empty(&self) == 1{
			return -1;
		}
		
		self.array[self.back as usize]
	}
	
	fn size(&self) -> i32 {
		if self.front <= self.back {
			self.back - self.front
		}
		else {
			self.max_length - self.front + self.back
		}
	}
}

fn command(queue: &mut Queue, op: &str) -> i32 {
	if op.starts_with("push") {
		let n: i32 = op.trim().
			split(" ").
			collect::<Vec<&str>>()[1]
			.	
			parse::<i32>().
			unwrap();
		queue.push(n);
		return -5;
	}
	match op {
		"pop" => queue.pop(),
		"size" => queue.size(),
		"empty" => queue.empty(),
		"front" => queue._front(),
		"back" => queue._back(),
		_ => -5,
	}
}

fn main() {
	let mut queue = Queue::new();

	let mut n = String::new();
	io::stdin().read_line(&mut n).unwrap();
	let n: usize = n.trim().parse().unwrap();

	for i in 0..n {
		let mut op = String::new();
		io::stdin().read_line(&mut op).unwrap();
		let result: i32 = command(&mut queue, &op.trim()[..]);
		if result != -5 {
			println!("{}", result);
		}
	}
}
