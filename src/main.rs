#![warn(
	clippy::all,
	clippy::pedantic,
)]

use std::str::from_utf8;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Whether or not to print initial and intermediate steps
	#[arg(short, long)]
	verbose: bool,

	/// The index of the term to compute
	#[arg(short, long)]
	n: u32,
}

/// ASCII code for '1'
const _1: u8 = 49;
/// ASCII code for '2'
const _2: u8 = 50;
/// ASCII code for '3'
const _3: u8 = 51;

/// Returns the next term of a given term.
///
/// # Arguments
///
/// * `prev` - A state to compute the next from
///
/// # Notes
///
/// Invalid input containing elements other than 49, 50, and 51 will produce erroneous output containing only of 49, 50, or 51.
fn next_step(prev: &Vec<u8>) -> Vec<u8> {
	let mut next = vec![];
	let mut i: usize = 0;

	for _ in 0..prev.len() {
		if i < prev.len().saturating_sub(2) && prev[i] == prev[i + 1] && prev[i] == prev[i + 2] {
			next.push(_3);
			next.push(prev[i]);
			i += 3;

			if i < prev.len() {
				continue;
			}

			break;
		}

		if i < prev.len().saturating_sub(1) && prev[i] == prev[i + 1] {
			next.push(_2);
			next.push(prev[i]);
			i += 2;

			if i < prev.len() {
				continue;
			}

			break;
		}

		next.push(_1);
		next.push(prev[i]);
		i += 1;

		if i >= prev.len() {
			break;
		}
	}

	next
}

/// Returns the nth term of the sequence.
///
/// # Arguments
///
/// * `n` - An index
/// * `print_intermediate` - Whether or not to print initial and intermediate steps
#[inline]
fn compute_nth(n: u32, print_intermediate: bool) -> Vec<u8> {
	(1..n).fold(vec![_1], |acc: Vec<u8>, _: u32| {
		if print_intermediate {
			println!("{}", from_utf8(&acc).unwrap());
		}

		next_step(&acc)
	})
}

fn main() {
	let args = Args::parse();
	let result = compute_nth(args.n, args.verbose);

	if args.verbose {
		println!("{}", from_utf8(&result).unwrap());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(compute_nth(1, false), [49]); // "1"
	}

	#[test]
	fn test_2() {
		assert_eq!(compute_nth(2, false), [49, 49]); // "11"
	}

	#[test]
	fn test_3() {
		assert_eq!(compute_nth(3, false), [50, 49]); // "21"
	}

	#[test]
	fn test_4() {
		assert_eq!(compute_nth(4, false), [49, 50, 49, 49]); // "1211"
	}

	#[test]
	fn test_10() {
		assert_eq!(compute_nth(10, false), [49, 51, 50, 49, 49, 51, 49, 49, 49, 50, 51, 49, 49, 51, 49, 49, 50, 50, 49, 49]); // "13211311123113112211"
	}
}
