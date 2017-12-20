extern crate load_input;

fn calculate_buffer(inserts: usize, step: usize) -> Vec<usize> {
	let mut buffer = std::iter::repeat(Default::default()).take(inserts + 1).collect::<Vec<Node>>();
	let mut pos = 0;
	for inserted in 0..inserts {
		if inserted % 1_000_000 == 0 {
			println!("Done {} steps!", inserted);
		}
		for _ in 0..step {
			pos = unsafe { buffer.get_unchecked(pos) } .next;
		}

		let n = inserted + 1;
		{
			let next = unsafe { buffer.get_unchecked(pos) }.next;
			let node = unsafe { buffer.get_unchecked_mut(n) };
			node.next = next;
			node.value = n;
		}

		unsafe { buffer.get_unchecked_mut(pos) }.next = n;
		pos = n;
	}

	println!("Done generating, moving on...");
	let mut result = Vec::with_capacity(inserts);
	result.push(0);
	pos = buffer[0].next;
	while pos != 0 {
		let nd = buffer[pos];
		result.push(nd.value);
		pos = nd.next;
	}

	result
}

#[derive(Copy, Clone, Default)]
struct Node {
	next: usize,
	value: usize
}

fn main() {
	let input = load_input::load_input();
	let step = input.trim().parse::<usize>().expect("A number");


	let state = calculate_buffer(2017, step);
	let first = state[0];
	let after_2017 = state.into_iter().skip_while(|&cur|cur != 2017).skip(1).next().unwrap_or(first);
	println!("After 2017: {}", after_2017);

	let state = calculate_buffer(50000000, step);
	let first = state[0];
	let after_50m = state.into_iter().skip_while(|&cur|cur != 0).skip(1).next().unwrap_or(first);
	println!("After 0, after 50m steps {}", after_50m);
}
