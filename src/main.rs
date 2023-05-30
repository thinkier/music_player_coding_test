#![feature(test)]
extern crate test;

mod bench;
mod tests;

fn main() {
    println!("Hello, world!");
}

const MODULO: usize = 60;

pub fn nested_for(s: &[usize]) -> usize {
	let mut answer = 0;
	for i in 0..s.len() { 
		let a = s[i];
		for b in s.iter().skip(i + 1) {
			if (a + b) % MODULO == 0 {
				answer += 1;
			}
		}
	}

	return answer;
}

pub fn pivot(s: &[usize]) -> usize {
	let mut answer = 0usize;

	let mut sorted = [0usize; MODULO];

	for x in s {
		sorted[x % MODULO] += 1;
	}

	// 0 and 30 case, base cases do not need to consult the pivot table
	for x in [0, MODULO / 2] {
		for i in 1..sorted[x] {
			answer += i;
		}
	}

	// Iterate over 1..30 then select the counterpart in 31..60.rev()
	for i in 1..MODULO / 2 {
		let j = MODULO - i;

		let a = sorted[i];
		let b = sorted[j];

		answer += a * b;
	}

	return answer;
}

pub fn songs(n: usize) -> Vec<usize> {
	let mut double_rand = vec![0u8; n * 2];

	getrandom::getrandom(&mut double_rand).ok();

	double_rand[0..n].iter()
		.zip(double_rand[n..].iter())
		.map(|(a, b)| ((*a as u16) << 8) | (*b as u16))
		.map(|x| x as usize)
		.collect()
}
