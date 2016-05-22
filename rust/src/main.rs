#![feature(test)]
extern crate mioco;
extern crate test;

use std::thread;

fn main() {

	channel(10);
}

pub fn channel(num_threads: u64) {
	let (tx, rx) = mioco::sync::mpsc::channel::<u64>();

	mioco::start(move || {

		for i in 0u64..num_threads {

			let tx = tx.clone();

			mioco::spawn(move || {
				tx.send(i).unwrap();
			});
		}

		for _ in 0u64..num_threads {
			let _ = rx.recv().unwrap();
		}
	}).unwrap();
}

pub fn channel_threads(num_threads: u64) {
	let (tx, rx) = std::sync::mpsc::channel::<u64>();

		for i in 0u64..num_threads {

			let tx = tx.clone();

			thread::spawn(move || {
				tx.send(i).unwrap();
			});
		}

		for _ in 0u64..num_threads {
			let _ = rx.recv().unwrap();
		}
}

#[cfg(test)]
mod tests {
	use super::*;
    use test::Bencher;

	#[bench]
	fn bench_rust_channel_10_u64(b: &mut Bencher) {
		b.iter(|| channel(10));
	}

	#[bench]
	fn bench_rust_channel_10000_u64(b: &mut Bencher) {
		b.iter(|| channel(10000));
	}

	#[bench]
	fn bench_rust_channel_threads_10_u64(b: &mut Bencher) {
		b.iter(|| channel_threads(10));
	}

	#[bench]
	fn bench_rust_channel_threads_10000_u64(b: &mut Bencher) {
		b.iter(|| channel_threads(10000));
	}
}
